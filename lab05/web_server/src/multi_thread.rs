use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs,
    path::Path,
    sync::{mpsc, Arc},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub fn run() {
    // 创建静态文件目录
    if !Path::new("static").exists() {
        fs::create_dir("static").unwrap();
        
        fs::write("static/hello.html", 
            "<!DOCTYPE html><html><head><title>Hello</title></head><body><h1>Hello from multi-thread server!</h1></body></html>"
        ).unwrap();
        
        fs::write("static/404.html", 
            "<!DOCTYPE html><html><head><title>404</title></head><body><h1>404 Not Found</h1></body></html>"
        ).unwrap();
    }

    // 监听端口
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("多线程服务器运行在 http://127.0.0.1:7878");

    // 创建线程池
    let pool = ThreadPool::new(num_cpus::get());

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(std::sync::Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<std::sync::Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} 处理请求", id);
            job();
        });

        Worker { id, thread }
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