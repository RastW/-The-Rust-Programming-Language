use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, fs};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for  stream in listener.incoming() {
        let stream = stream.unwrap();
    
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    
    stream.read(&mut buffer).unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // 请求
    // Method Request-URI HTTP-version CRLF

    // let contents = fs::read_to_string("hello.html").unwrap();
    // let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    // let response = "HTTP/1.1 200 OK\r\n\r\n";

    let contents = fs::read_to_string("hello.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    // println!("{response}");

    stream.write(response.as_bytes()).unwrap();
    // 等待并阻止程序的运行，直到所有数据被写入
    stream.flush().unwrap();
}
