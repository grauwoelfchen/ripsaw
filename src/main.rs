extern crate base64;
extern crate serde;
extern crate serde_json;

use std::env;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use base64::decode;
use serde::Deserialize;
use serde_json::Result;

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

fn handle<'a>(input: &'a str) -> Result<()> {
    println!("input: {}", input);

    let m: PubSubMessage<'a> = serde_json::from_str(input)?;

    println!("subscription: {}", m.subscription);

    let data = &decode(m.message.data).unwrap()[..];
    println!("data: {}", String::from_utf8_lossy(data));
    println!("message_id: {}", m.message.message_id);
    println!("publish_time: {}", m.message.publish_time);

    Ok(())
}

fn handle_request(mut stream: TcpStream) {
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
            let _ = handle(line);

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
                thread::spawn(move || handle_request(s));
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
