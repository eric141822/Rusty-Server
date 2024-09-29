use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread, time::Duration
};
use rusty_server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);


    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => {
            eprintln!("Failed to read line: {}", e);
            return;
        }
        None => {
            eprintln!("No lines in buffer");
            return;
        }
    };

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };

    let len = contents.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, len, contents
    );

    match stream.write_all(response.as_bytes()) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to write to stream: {}", e),
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <num_requests>");
        return;
    }

    let num_requests: usize = match args[1].parse() {
        Ok(n) => {
            assert!(n > 0);
            println!("Server will only serve {} requests", n);
            n
        },
        Err(_) => {
            eprintln!("Please provide a valid number for num_requests");
            return;
        }
    };
    

    let listener = match TcpListener::bind("127.0.0.1:8080") {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to port 8080: {}", e);
            return;
        }
    };
    let pool = ThreadPool::new(4);

    // only serve 2 requests before shutting down
    for stream in listener.incoming().take(num_requests) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    
}
