//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 1+
//
// Note that this code has serious security risks! You should not run it
// on any system with access to sensitive files.
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;
use std::thread;
use std::path::Path;
use std::fs::File;

use std::error::Error;

use std::env;

static mut requests:u32 = 0;

fn main() {
    let addr = "127.0.0.1:4414";

    let listener = TcpListener::bind(addr).unwrap();

    println!("Listening on [{}] ...", addr);

    for stream in listener.incoming() {
        match stream {
            Err(_) => (),
            Ok(mut stream) => {
                // Spawn a thread to handle the connection
                thread::spawn(move|| {
                    match stream.peer_addr() {
                        Err(_) => (),
                        Ok(pn) => println!("Received connection from: [{}]", pn),
                    }
                    unsafe {
                        requests += 1;
                    }

                    let mut buf = [0 ;500];
                    stream.read(&mut buf).unwrap();
                    let response;
                    match str::from_utf8(&buf) {
                        Err(error) => {
                            println!("Received request error:\n{}", error);
                            response = default_response();
                        }
                        Ok(body) => {
                            println!("Recieved request body:\n{}", body);
                            response = process_request(body);
                        }
                    }

                   stream.write(response.as_bytes()).unwrap();
                    println!("Connection terminates.");
                });
            },
        }
    }

    drop(listener);
}

fn process_request(s: &str) -> String {
    let lines: Vec<&str> = s.lines().collect();
    let request: Vec<&str> = lines[0].split_whitespace().collect();
    let req_type = request[0];
    let path = request[1];

    if req_type != "GET" {
        return String::from("HTTP/1.1 501 Not Implemented\r\n");
    }
    
    if path != "/" {
        //format!("HTTP/1.1 200 OK\r\nContent-Type: text/html;\r\n\r\n{:?}",lines)
        respond_file(path)
    } else {
        default_response()
    }
}

fn respond_file(req_path: &str) -> String {
    let p = Path::new(req_path);
    match p.extension() {
        None => return response_permission_denied(),
        Some(ext) => if ext != "html" {
            return response_permission_denied();
        }
    }
    
    // try to open the file. if it doesn't exist, respond with 404. 
    if let Ok(mut cwd) = env::current_dir() {
        cwd.push(&req_path[1..]);
        let mut f =  match File::open(cwd) {
            Err(e) => return response_not_found(e.description()),
            Ok(fh) => fh
        };
        let mut buf = String::new();
        match f.read_to_string(&mut buf) {
            Err(e) => response_server_error(e.description()),
            Ok(bytes_read) => response_ok(&buf) 
        }
    } else {
        response_not_found("")
    }
}

fn response_permission_denied() -> String {
    http_response("403 Forbidden","Forbidden")
}

fn response_server_error(content: &str) -> String {
    http_response("501 Server Error", content)
}

fn response_ok(content: &str) -> String {
    http_response("200 OK", content)
}

fn response_not_found(content: &str) -> String {
    http_response("404 Not Found",content)
}

fn http_response(r: &str, b: &str) -> String {
   format!("HTTP/1.1 {}\r\n\r\n{}", r, b)
}

fn default_response() -> String {
    let response_head =
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
         <doctype !html><html><head><title>Hello, Rust!</title>
         <style>body { background-color: #111; color: #FFEEAA }
                h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
         </style></head>
         <body>
         <h1>Greetings, Krusty!</h1>";
    let request_count;
    unsafe {    
        request_count = format!("<h2>Requests: {}</h2>",requests); 
    }
    let footer = "</body></html>\r\n";
    let response = format!("{}{}{}",response_head,request_count,footer);
    response
}
