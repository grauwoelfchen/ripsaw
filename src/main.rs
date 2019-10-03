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
use oauth::{GetToken, ServiceAccountAccess};
use hyper::Client as HttpClient;
use hyper_rustls::TlsClient;
use hyper::net::HttpsConnector;
use serde::Deserialize;
use serde_json::Result as JsonResult;
use storage::{ObjectMethods, Storage};

const SA_KEY_FILE: &str = "key.json";
const OAUTH_SCOPE: &str = "https://www.googleapis.com/auth/devstorage.read_write";
const BUCKET_NAME: &str = "ripsaw";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Attributes<'a> {
    _bucket_id: &'a str,
    _event_time: &'a str,
    event_type: &'a str,
    object_id: &'a str,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data<'a> {
    _id: &'a str,
    kind: &'a str,
    name: &'a str,
    _size: &'a str,
    bucket: &'a str,
    content_type: &'a str,
    time_created: &'a str,
}

#[derive(Deserialize)]
struct Message<'a> {
    data: &'a [u8],
    attributes: Attributes<'a>,
    message_id: &'a str,
    _publish_time: &'a str,
}

#[derive(Deserialize)]
struct PubSubMessage<'a> {
    message: Message<'a>,
    _subscription: &'a str,
}

type Methods<'a> = ObjectMethods<'a, HttpClient, ServiceAccountAccess<HttpClient>>;

fn handle<'a>(input: &'a str, methods: &Methods) -> JsonResult<()> {
    println!("input: {}", input);

    let m: PubSubMessage<'a> = serde_json::from_str(input)?;

    let content = &decode(m.message.data).unwrap()[..];
    let data = String::from_utf8_lossy(content);

    println!("message_id: {}", m.message.message_id);

    println!("event_type: {}", m.message.attributes.event_type);
    println!("object_id: {}", m.message.attributes.object_id);

    let d: Data = serde_json::from_str(&data)?;

    println!("kind: {}", d.kind);
    println!("content_type: {}", d.content_type);
    println!("bucket: {}", d.bucket);

    if d.kind == "storage#object" && d.content_type == "text/csv" &&
        d.bucket == BUCKET_NAME {
        println!("time created: {}", d.time_created);

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
                    let secret = oauth::service_account_key_from_file(
                        &SA_KEY_FILE.to_string()).unwrap();
                    let client = HttpClient::with_connector(
                        HttpsConnector::new(TlsClient::new()));

                    let mut access = ServiceAccountAccess::new(secret, client);
                    access.token(&vec![OAUTH_SCOPE]).unwrap();

                    let client = HttpClient::with_connector(
                        HttpsConnector::new(TlsClient::new()));
                    let storage = Storage::new(client, access);
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
