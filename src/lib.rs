// lib.rs

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

// 用户
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String, // 姓名
    pub email: String,    // 邮箱
}

// 消息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub sender: String,   // 发送者
    pub receiver: String, // 接收者
    pub content: String,  // 内容
}

// 好友
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Friend {
    pub user: String,   // 用户
    pub friend: String, // 好友
}

// 通讯记录
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatHistory {
    pub sender: String,         // 发送者
    pub receiver: String,       // 接收者
    pub messages: Vec<Message>, // 消息列表
}

pub fn get_chat_history(sender: &str, receiver: &str) -> Result<ChatHistory, String> {
    // 从区块链上检索与指定用户相关的消息
    // TODO 区块链交互的逻辑

    let messages: Vec<Message> = vec![]; // 从区块链获取消息

    Ok(ChatHistory {
        sender: sender.to_string(),
        receiver: receiver.to_string(),
        messages,
    })
}

// 允许用户搜索通讯记录
pub fn search_messages(chat_history: &ChatHistory, keyword: &str) -> Vec<Message> {
    chat_history
        .messages
        .iter()
        .filter(|message| message.content.contains(keyword))
        .cloned()
        .collect()
}

// 用户导出通讯记录
pub fn export_chat_history(chat_history: &ChatHistory, file_path: &str) -> Result<(), String> {
    let file: File = File::create(file_path).map_err(|e| format!("创建文件失败：{}", e))?;
    let mut writer = std::io::BufWriter::new(file);

    let content = serde_json::to_string_pretty(chat_history)
        .map_err(|e| format!("序列化通讯记录失败：{}", e))?;

    writer
        .write_all(content.as_bytes())
        .map_err(|e| format!("写入文件失败：{}", e))?;

    Ok(())
}
