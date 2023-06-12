use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    println!("Listening on port 7878");
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_count = Arc::new(Mutex::new(0 as usize));
    let max_threads: usize = 2;

    let mut handles = vec![];

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        let mut temp_thread_count = thread_count.lock().unwrap();

        if *temp_thread_count < max_threads {
            *temp_thread_count += 1;
            drop(temp_thread_count);
            let thread_count = Arc::clone(&thread_count);
            let handle = thread::spawn(move || {
                handle_connection(stream);
                let mut thread_count = thread_count.lock().unwrap();
                *thread_count -= 1;
            });
            handles.push(handle);
        } else {
            reject_connection(stream);
        }
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
fn reject_connection(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 501 INTERNAL SERVER ERROR";
    let contents = "Server is currently experiencing too many requests";
    let content_length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    thread::sleep(std::time::Duration::from_secs(3));

    let status_line = "HTTP/1.1 200 OK";
    let contents = "Hello World";
    let content_length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
