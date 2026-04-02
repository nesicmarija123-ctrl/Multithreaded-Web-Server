use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*; //contents inside of prelude module

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = 
    if buffer.starts_with(get){
        ("HTTP/1.1 200 OK", "index.html")
    }
    else{
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();

       let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}