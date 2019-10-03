extern crate base64;
extern crate google_storage1 as storage;
extern crate hyper;
extern crate hyper_rustls;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth;

use std::env;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use base64::decode;
use oauth::{ApplicationSecret, Authenticator, DefaultAuthenticatorDelegate,
    MemoryStorage};
use hyper::Client as HttpClient;
use hyper_rustls::TlsClient;
use hyper::net::HttpsConnector;
use serde::Deserialize;
use serde_json::Result as JsonResult;
use storage::{ObjectMethods, Storage};

const BUCKET_NAME: &str = "ripsaw";

#[derive(Deserialize)]
struct Message<'a> {
    data: &'a [u8],
    message_id: &'a str,
    publish_time: &'a str,
}

#[derive(Deserialize)]
struct PubSubMessage<'a> {
    message: Message<'a>,
    subscription: &'a str,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data<'a> {
    id: &'a str,
    kind: &'a str,
    name: &'a str,
    size: &'a str,
    bucket: &'a str,
    content_type: &'a str,
    time_created: &'a str,
}

type Methods<'a> = ObjectMethods<'a, HttpClient, Authenticator<
  DefaultAuthenticatorDelegate, MemoryStorage, HttpClient>>;

fn handle<'a>(input: &'a str, methods: &Methods) -> JsonResult<()> {
    println!("input: {}", input);

    let m: PubSubMessage<'a> = serde_json::from_str(input)?;

    println!("subscription: {}", m.subscription);

    let content = &decode(m.message.data).unwrap()[..];
    let data = String::from_utf8_lossy(content);

    println!("data: {}", data);
    println!("message_id: {}", m.message.message_id);
    println!("publish_time: {}", m.message.publish_time);

    let d: Data = serde_json::from_str(&data)?;
    if d.kind == "storage#object" && d.content_type == "textv/csv" &&
        d.bucket == BUCKET_NAME {
        let result = methods.get(&BUCKET_NAME, d.name).doit();
        match result {
            Err(e) => {
                println!("err: {}", e);
            },
            Ok((mut response, _object)) => {
                let mut buffer = String::new();
                let _ = response.read_to_string(&mut buffer);
                println!("response: {}", buffer);
            },
        }

        println!("id: {}", d.id);
        println!("size: {}", d.size);
        println!("time created: {}", d.time_created);
    }

    Ok(())
}

fn handle_request(mut stream: TcpStream, methods: &Methods) {
    stream
        .set_nonblocking(true)
        .expect("cannot set non-blocking mode");
    stream.set_nodelay(true).expect("cannot set nodelay mode");

    let mut buf = String::new();
    let _size = match stream.read_to_string(&mut buf) {
        Ok(s) => s,
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => 0,
        Err(e) => {
            panic!("err: {}", e);
        },
    };

    let mut has_body = false;
    for line in buf.lines() {
        if line == "" {
            has_body = true;
        } else if has_body {
            // TODO
            let _ = handle(line, methods);

            // echo
            let response = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", line);
            stream
                .write_all(response.as_bytes())
                .unwrap();
            stream.flush().unwrap();
        }
    }
}

fn get_addr() -> String {
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    format!("{}:{}", host, port)
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(get_addr())?;

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || {
                    // FIXME
                    // I'd like to authenticate once before spawn :'(
                    let secret: ApplicationSecret = Default::default();
                    let auth = Authenticator::new(
                        &secret,
                        DefaultAuthenticatorDelegate,
                        HttpClient::with_connector(
                            HttpsConnector::new(TlsClient::new())),
                        <MemoryStorage as Default>::default(),
                        None,
                    );
                    let storage = Storage::new(
                        HttpClient::with_connector(
                            HttpsConnector::new(TlsClient::new())),
                        auth,
                    );
                    let o = storage.objects();

                    handle_request(s, &o);
                });
            },
            Err(e) => panic!("err: {}", e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod main {
    use super::*;

    #[test]
    fn test_get_addr() {
        let addr = get_addr();
        assert_eq!("0.0.0.0:8000", addr);
    }
}
