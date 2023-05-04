// main.rs
mod client;
mod server;

use std::fs::File;
use std::io::Read;
use toml::Value;

fn main() {
    let mut config_file = File::open("config.toml").expect("配置文件未找到");
    let mut config_toml = String::new();
    config_file
        .read_to_string(&mut config_toml)
        .expect("读取配置文件出错");

    let config: Value = config_toml.parse().expect("解析配置文件出错");

    let mode = config
        .get("settings")
        .and_then(|s| s.get("mode"))
        .and_then(|m| m.as_str());

    match mode {
        Some("server") => {
            println!("运行为服务端");
            server::run_server();
        }
        Some("client") => {
            println!("运行为客户端");
            client::run_client();
        }
        Some(_) => println!("无效的运行模式，请检查配置文件"),
        None => println!("配置文件中未指定运行模式"),
    }
}
