use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread, time::Duration,
};

fn handle_stream(mut stream: TcpStream) {
    let mut buf = vec![0; 1500];

    let handle = thread::spawn(move || {
        loop {
            let num_of_bytes = stream.read(&mut buf).unwrap();
            // EOF of stream, client shutdown
            if num_of_bytes == 0 {
                break;
            };
            let thread_id = thread::current().id();
            let contents = std::str::from_utf8(&buf[..num_of_bytes]).unwrap();
            println!("thread {thread_id:?} number of bytes received: {num_of_bytes}, contents: {contents}");
            // write response
            thread::sleep(Duration::from_secs(1));
            stream.write(b"OK").unwrap();
            // thread::sleep(Duration::from_secs(1));
            // stream.write(b"OK").unwrap();

        }

        println!("client shutdown");
    });
}

/// TCP server to simulate slow response
///
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Waiting for incoming connections");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_stream(stream),
            Err(error) => println!("connection failure: {}", error),
        }
    }
}
