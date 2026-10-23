use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello::ThreadPool;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listner.incoming().take(2) {
        let stream = stream.unwrap();
        pool.excute(|| {
            handel_connection(stream);
        });
    }

    println!("Shitting Donw");
}

fn handel_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let reqeust_line = &buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_name) = match &reqeust_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello copy.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "not_found.html"),
    };
    let content = fs::read_to_string(file_name).unwrap();
    let length = content.len();

    let response = format!("{status_line}\r\nContet_Length:{length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}

// let http_request: Vec<_> = reqeust_line
//     .lines()
//     .map(|result| result.unwrap())
//     .take_while(|line| !line.is_empty())
//     .collect();

// let status_line = "HTTP/1.1 200 OK\r\n\r\n";
// let content = fs::read_to_string("hello.html").unwrap();
// let length = content.len();
