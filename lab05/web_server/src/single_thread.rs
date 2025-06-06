// src/single_thread.rs
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs,
    path::Path,
};

pub fn run() {
    // 监听本地7878端口
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("单线程服务器运行在 http://127.0.0.1:7878");

    // 创建静态文件目录
    create_static_dir();

    // 处理连接
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "static/hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "static/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        fs::read_to_string("static/404.html").unwrap()
    });
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}

fn create_static_dir() {
    if !Path::new("static").exists() {
        fs::create_dir("static").unwrap();
        
        // 创建默认HTML文件
        fs::write("static/hello.html", 
            "<!DOCTYPE html><html><head><title>Hello</title></head><body><h1>Hello from single-thread server!</h1></body></html>"
        ).unwrap();
        
        fs::write("static/404.html", 
            "<!DOCTYPE html><html><head><title>404</title></head><body><h1>404 Not Found</h1></body></html>"
        ).unwrap();
    }
}