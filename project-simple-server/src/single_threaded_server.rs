// REF: https://doc.rust-lang.org/book/ch20-01-single-threaded.html

use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // // println!("request: {http_request:#?}");

    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap(); // NOTE: review

    // if request_line == "GET / HTTP/1.1" {
    //     let status_line = "HTTP/1.1 200 OK";
    //     let contents = fs::read_to_string("./data/hello.html").unwrap();
    //     let length = contents.len();
    //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //     stream.write_all(response.as_bytes()).unwrap();
    // } else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("./data/404.html").unwrap();
    //     let length = contents.len();
    //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //     stream.write_all(response.as_bytes()).unwrap();
    // }

    // let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "./data/hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "./data/404.html")
    // };

    // NOTE: review
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./data/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            (
                "HTTP/1.1 301 Moved Permanently Location: ./",
                "./data/404.html",
            )
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "./data/404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
