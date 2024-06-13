use core::panic;
use std::{
    fs::{self},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream,},             //TODO: add this functionality if required  IpAddr, Ipv4Addr
    process::Command,

};
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let port_number = std::env::var("PORT").expect("PORT number must be set.");
    //let _username = std::env::var("USERNAME").expect("USERNAME must be set.");                  //TODO: this has the password (remove the "_")
    //let _password = std::env::var("PASSWORD").expect("USERNAME must be set.");                  //TODO: this has the username

    let ipaddr: String = local_ip_detect();

    let listener = TcpListener::bind(format!("0.0.0.0:{port_number}")).unwrap();
    
    println!("Server running on http://{ipaddr}:{port_number}");
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
    } else 
    if request_line == "GET /ping HTTP/1.1" {
        match stream.write_all("pong 🏓".as_bytes()) {
            Ok(s) => s,
            Err(er) => println!("Error in writing to the stream: {}", er)
        };
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

fn local_ip_detect() -> String {
    let os = std::env::var("OS").expect("Operating system env variable (OS) must be set.");
    let output = 
    if cfg!(target_os = "linux") && os == "ARCH" 
    {
        Command::new("sh")
            .arg("-c")
            .arg("ip addr")
            .output()
            .expect("failed to execute process")
    } else 
    if cfg!(target_os = "linux") && os == "UBUNTU" 
    {
        Command::new("sh")
            .arg("-c")
            .arg("echo helloi")
            .output()
            .expect("failed to execute process")
    }
    else {
        Command::new("sh")
            .arg("-c")
            .arg("echo either use ubuntu or arch. Also, if you are aren't on linux yet, wtf are you doing brother.")
            .output()
            .expect("failed to execute process")
    };
    let stdout = String::from_utf8(output.stdout).unwrap();
    println!("{}", stdout);
    "balls".to_string()
    
}
    