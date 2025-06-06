use std::io;

mod single_thread;
mod multi_thread;
mod async_server;

#[tokio::main]  // 即使选择非异步版本也会保留，对同步代码无害
async fn main() -> io::Result<()> {
    println!("请选择服务器类型:");
    println!("1. 单线程版本");
    println!("2. 多线程版本");
    println!("3. 异步版本(Tokio)");
    println!("输入数字后按回车:");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    match choice.trim() {
        "1" => {
            println!("启动单线程服务器...");
            single_thread::run();
        }
        "2" => {
            println!("启动多线程服务器...");
            multi_thread::run();
        }
        "3" => {
            println!("启动异步服务器...");
            async_server::run().await;
        }
        _ => {
            eprintln!("无效输入，请输入1-3的数字");
            std::process::exit(1);
        }
    }

    Ok(())
}