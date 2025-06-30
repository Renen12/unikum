use crate::{return_server_values, return_server_values_messages, server::threadpool::ThreadPool};
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
            eprintln!("No request");
            return;
        }
    };
    let request_base = &**request_base;
    let request_split: Vec<&str> = request_base.split(" ").collect();
    let url = match request_split.get(1) {
        Some(v) => v,
        None => {
            eprintln!("No request can be built");
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
    let mut return_messages = false;
    for pair in &pairs {
        let split: Vec<&str> = pair.split("=").collect();
        let key = &**(match split.get(0) {
            Some(v) => v,
            None => {
                eprintln!("No key");
                return;
            }
        });
        let value = &**(match split.get(1) {
            Some(v) => v,
            None => {
                eprintln!("No value: {:?}", split);
                return;
            }
        });
        match key {
            "jsess" => jsess = value.to_owned(),
            "uni" => uni = value.to_owned(),
            "shibn" => shibn = value.to_owned(),
            "shibv" => shibv = value.to_owned(),
            "pid" => pid = value.to_owned(),
            "messages" => {
                if value == "true" {
                    return_messages = true
                } else {
                    return_messages = false
                }
            }
            _ => eprintln!("Unrecognised key and value: {}", pair),
        }
        if !jsess.is_empty()
            && !uni.is_empty()
            && !shibn.is_empty()
            && !shibv.is_empty()
            && !pid.is_empty()
        {
            println!("Returning posts");
            let mut result = return_server_values(&jsess, &uni, &shibn, &shibv, &pid);
            result = result.replace("\n", "");
            let json_value: serde_json::Value = match serde_json::from_str(&result) {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Invalid JSON");
                    return;
                }
            };
            let body = match serde_json::to_string(&json_value) {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Cannot turn JSON into string");
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
            let _json: serde_json::Value = match serde_json::from_str(&result)
                .map_err(|e| format!("JSON parse error: {}", e))
            {
                Ok(v) => v,
                Err(_) => {
                    return;
                }
            };
            let _ = stream.write_all(response.as_bytes());
            return;
        } else if return_messages {
            println!("Returning messages");
            let mut bearer = String::from("Not set");
            for pair in &pairs {
                let split: Vec<&str> = pair.split("=").collect();
                let key = &**(match split.get(0) {
                    Some(v) => v,
                    None => {
                        eprintln!("No key");
                        return;
                    }
                });
                let value = &**(match split.get(1) {
                    Some(v) => v,
                    None => {
                        eprintln!("No value: {:?}", split);
                        return;
                    }
                });
                if key == "bearer" || key.contains("bearer") {
                    bearer = value.replace("%20", " ");
                }
            }
            let mut result = return_server_values_messages(&bearer);
            result = result.replace("\n", "");
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
            stream.write_all(response.as_bytes()).unwrap();
            return;
        }
    }
}
