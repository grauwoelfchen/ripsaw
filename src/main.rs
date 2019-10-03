use std::env;
use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle(mut stream: TcpStream) {
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
            println!("{}", line);
            let response = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", line);
            stream
                .write_all(response.as_bytes())
                .unwrap();
            stream.flush().unwrap();
        }
    }
    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
}

fn get_addr() -> String {
    let host = match env::var("HOST") {
        Ok(value) => value,
        Err(_) => "0.0.0.0".to_string(),
    };
    let port = match env::var("PORT") {
        Ok(value) => value,
        Err(_) => "8080".to_string(),
    };
    format!("{}:{}", host, port)
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(get_addr())?;

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || handle(s));
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
        assert_eq!("127.0.0.1:8080", addr);
    }
}
