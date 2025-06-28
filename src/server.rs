use crate::{return_server_values, server::threadpool::ThreadPool};
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    process::exit,
};
mod threadpool;
pub fn server() {
    let listener = match TcpListener::bind("127.0.0.1:7951") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Can't bind to port 7951: {}", e);
            exit(1);
        }
    };
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let pool = ThreadPool::new(4);
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
#[allow(unused)]
fn handle_connection(mut stream: std::net::TcpStream) {
    let unparsed: Vec<_> = BufReader::new(&stream)
        .lines()
        .map(|r| match r {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Cannot read from stream");
                String::new()
            }
        })
        .take_while(|line| !line.is_empty())
        .collect();
    let binding = unparsed.get(0);
    let request_base = match &binding {
        Some(v) => v,
        None => {
            return;
        }
    };
    let request_base = &**request_base;
    let request_split: Vec<&str> = request_base.split(" ").collect();
    let url = match request_split.get(1) {
        Some(v) => v,
        None => {
            return;
        }
    };
    let url = url.replacen("/?", "", 1);
    let pairs: Vec<&str> = url.split("&").collect();
    let mut pid = String::new();
    let mut jsess = String::new();
    let mut uni = String::new();
    let mut shibn = String::new();
    let mut shibv = String::new();
    for pair in pairs {
        let split: Vec<&str> = pair.split("=").collect();
        let key = &**(match split.get(0) {
            Some(v) => v,
            None => {
                return;
            }
        });
        let value = &**(match split.get(1) {
            Some(v) => v,
            None => {
                return;
            }
        });
        match key {
            "jsess" => jsess = value.to_owned(),
            "uni" => uni = value.to_owned(),
            "shibn" => shibn = value.to_owned(),
            "shibv" => shibv = value.to_owned(),
            "pid" => pid = value.to_owned(),
            _ => return,
        }
        if !jsess.is_empty()
            && !uni.is_empty()
            && !shibn.is_empty()
            && !shibv.is_empty()
            && !pid.is_empty()
        {
            let mut result = return_server_values(&jsess, &uni, &shibn, &shibv, &pid);
            result = result.replace("\n", "");
            let value_length = result.len();
            let json_value: serde_json::Value = match serde_json::from_str(&result) {
                Ok(v) => v,
                Err(_) => {
                    return;
                }
            };
            let body = match serde_json::to_string(&json_value) {
                Ok(v) => v,
                Err(_) => {
                    return;
                }
            };
            let length = body.len();
            let response = format!(
                "\
HTTP/1.1 200 OK\r\n\
Content-Type: application/json\r\n\
Content-Length: {length}\r\n\
Server: Custom Unikum API server\r\n\
Access-Control-Allow-Origin: *\r\n\
\r\n\
{body}"
            );
            let json: serde_json::Value = match serde_json::from_str(&result)
                .map_err(|e| format!("JSON parse error: {}", e))
            {
                Ok(v) => v,
                Err(_) => {
                    return;
                }
            };
            stream.write_all(response.as_bytes());
        }
    }
}
