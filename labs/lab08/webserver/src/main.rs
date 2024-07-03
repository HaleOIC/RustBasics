//! A simple web server
//! which serves some html at index.html
//! and replaces triple curly braces with the given variable
mod test;
use std::io::{Read, Write};

use std::net::{TcpListener, TcpStream};
// hint, hint
use std::sync::{Arc, Mutex};
use std::thread;

struct State {
    counter: i32,
}

fn handle_client(mut stream: TcpStream, mutex: Arc<Mutex<State>>) {
    // setup buffer, and read from stream into buffer
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // convert request payload to string
    let string = String::from_utf8_lossy(&buffer);

    // extract header
    let mut split = string.split("\r\n");
    let header = split.next().unwrap();

    if header == "POST /counter HTTP/1.1" {
        let mut inner = mutex.lock().unwrap();
        inner.counter += 1;
    }

    let counter;
    {
        let inner = mutex.lock().unwrap();
        counter = inner.counter;
    }

    let file = include_bytes!("../index.html");
    let mut content = String::from_utf8_lossy(file).to_string();

    // Replace triple curly braces with the counter value
    content = content.replace("{{{ counter }}}", &counter.to_string());

    let file = content.as_bytes();

    // DONT CHANGE ME
    let response = format!(
        "HTTP/1.1 200 OK\r\nContentType: text/html; charset=utf-8\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n",
        file.len()
    );

    // converts response to &[u8], and writes them to the stream
    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(file).unwrap();
    stream.flush().unwrap();
}

fn main() -> std::io::Result<()> {
    let port = std::env::args().nth(1).unwrap_or("12345".to_string());
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))?;

    println!("Server running on port {}", port);
    let mutex = Arc::new(Mutex::new(State { counter: 0 }));

    // accept connections and process them serially
    for stream in listener.incoming() {
        // spawn a thread for each connection
        let mutex = Arc::clone(&mutex);
        thread::spawn(move || {
            handle_client(stream.unwrap(), mutex);
        })
        .join()
        .unwrap();
    }
    Ok(())
}
