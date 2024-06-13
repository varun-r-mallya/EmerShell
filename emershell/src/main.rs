use core::panic;
use std::{
    fs::{self},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:1024").unwrap();

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(_) => {stream.unwrap()},
            Err(_) => {panic!("Error with the stream")},
        };
        handle_connection(stream);
    }    
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    match buf_reader.lines().next() {
        None => {
            println!("No lines in the request sent");
        },
        Some(request_content) => {
            match request_content {
                Ok(s) => {
                        println!("{}", s);
                        route_decider(s, stream);
                },
                Err(er) => {
                        println!("Error in reading request content: {} ", er);
                }
            }
        }

    };
}

fn route_decider(request_line: String, mut stream: TcpStream) {
    let (status_line, filename) = 
    if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "views/index.html")                 //login page
    } else 
    if request_line == "POST /login HTTP/1.1" {
        authoriser(stream);                                     //login API
        return;
    } else {
        ("HTTP/1.1 404 NOT FOUND", "views/404.html")            //error page
    };

    match stream.write_all(page_display(filename, status_line).as_bytes()) {
        Ok(s) => s,
        Err(er) => println!("Error in writing to the stream: {}", er)
    };
    println!("Request: {:#?}", request_line);
}

fn page_display(filename: &str, status_line: &str) -> String {
    let contents = fs::read_to_string(filename).expect("balls");
    
    let length = contents.len();

    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
}

fn authoriser(mut stream: TcpStream) {
    println!("authorise function ran")
}