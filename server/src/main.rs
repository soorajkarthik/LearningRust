use std::net::TcpListener;

use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("localhost:8000").unwrap();
    let pool = ThreadPool::new(10);

    for stream in listener.incoming().take(100) {
        let stream = stream.unwrap();
        pool.execute(|| server::handle_connection(stream));
    }

    println!("Shutting down.");
}