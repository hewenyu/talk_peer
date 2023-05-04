// client/mod.rs
use std::io::{stdin, Read, Write};
use std::net::TcpStream;
use std::thread;

pub fn run_client() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("无法连接到服务器");

    println!("已连接到服务器，请输入用户名：");

    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();
    username = username.trim().to_string();

    stream.write_all(username.as_bytes()).unwrap();
    stream.flush().unwrap();

    let mut stream_clone = stream.try_clone().unwrap();
    thread::spawn(move || loop {
        let mut buffer = [0; 1024];
        let bytes_read = stream_clone.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }

        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("{}", message);
    });

    loop {
        let mut message = String::new();
        stdin().read_line(&mut message).unwrap();
        message = message.trim().to_string();

        if message.is_empty() {
            continue;
        }

        stream.write_all(message.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
