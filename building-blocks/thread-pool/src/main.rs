use pool::ThreadPool;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4).unwrap();
    let mut counter = 0;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        counter += 1;
        if counter > 2 { // 只允许客户端建立两个连接
            println!("Shutting down the server");
            break;
        }

        pool.spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(stream: TcpStream) {
    println!("executing...");
}