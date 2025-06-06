use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    fs,
};
use std::path::Path;  // 这里改为使用标准库的Path

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // 创建静态文件目录
    if !Path::new("static").exists() {
        fs::create_dir("static").await?;
        
        fs::write("static/hello.html", 
            "<!DOCTYPE html><html><head><title>Hello</title></head><body><h1>Hello from async server!</h1></body></html>"
        ).await?;
        
        fs::write("static/404.html", 
            "<!DOCTYPE html><html><head><title>404</title></head><body><h1>404 Not Found</h1></body></html>"
        ).await?;
    }

    // 监听端口
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("异步服务器运行在 http://127.0.0.1:7878");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            
            if let Err(e) = socket.read(&mut buf).await {
                eprintln!("读取错误: {}", e);
                return;
            }

            let get = b"GET / HTTP/1.1\r\n";
            let (status_line, filename) = if buf.starts_with(get) {
                ("HTTP/1.1 200 OK", "static/hello.html")
            } else {
                ("HTTP/1.1 404 NOT FOUND", "static/404.html")
            };
            
            let contents = match fs::read_to_string(filename).await {
                Ok(c) => c,
                Err(_) => match fs::read_to_string("static/404.html").await {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("文件读取错误: {}", e);
                        return;
                    }
                }
            };
            
            let response = format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                contents.len(),
                contents
            );
            
            if let Err(e) = socket.write_all(response.as_bytes()).await {
                eprintln!("写入错误: {}", e);
            }
        });
    }
}