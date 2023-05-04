// server/mod.rs
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// 在线用户列表类型定义
type Users = Arc<Mutex<HashMap<String, TcpStream>>>;

pub fn run_server() {
    let users: Users = Arc::new(Mutex::new(HashMap::new()));
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("服务器正在运行，等待客户端连接...");

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        let users: Arc<Mutex<HashMap<String, TcpStream>>> = Arc::clone(&users);
        thread::spawn(move || {
            handle_client(stream, users);
        });
    }
}

fn handle_client(mut stream: TcpStream, users: Users) {
    let mut buffer = [0; 1024];
    let mut username: String = String::new();

    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }

        let message: std::borrow::Cow<str> = String::from_utf8_lossy(&buffer[..bytes_read]);

        if username.is_empty() {
            username = message.trim().to_string();
            users
                .lock()
                .unwrap()
                .insert(username.clone(), stream.try_clone().unwrap());
            println!("用户 {} 已上线", username);
            send_online_users(&users, &username);
        } else {
            println!("来自 {} 的消息：{}", username, message);
            // 处理 P2P 打洞和消息转发逻辑
        }
    }

    println!("用户 {} 已下线", username);
    users.lock().unwrap().remove(&username);
    send_online_users(&users, &username);
}

fn send_online_users(users: &Users, exclude: &str) {
    let users_list = users
        .lock()
        .unwrap()
        .keys()
        .filter(|user| *user != exclude)
        .cloned()
        .collect::<Vec<String>>()
        .join(",");

    for (_, stream) in users.lock().unwrap().iter_mut() {
        stream.write_all(users_list.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
