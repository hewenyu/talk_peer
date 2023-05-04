use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;


fn main() {
    // start listen
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    println!("Server started at :8080");

    // wait incoming connection
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        // handle connection

        if let Err(e) = handle_connection(stream) {
            println!("Error: {}", e);
        }
    }
}

// function to handle connection
fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    // read request
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    // parse request
    let request = String::from_utf8_lossy(&buffer[..]);
    let request = request.split_whitespace().collect::<Vec<&str>>();

    // get path
    let path = request[1];

    // get file
    let mut file = File::open(&path[1..])?;

    // read file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // write response
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
