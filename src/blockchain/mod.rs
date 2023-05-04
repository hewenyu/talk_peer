// blockchain/mod.rs

// use hyper::service::{make_service_fn, service_fn};
// use hyper::Client;
// use hyper::Uri;
// use hyper::{service, Body, Request, Response};
// use serde::{Deserialize, Serialize};
// use std::convert::TryFrom;
// use std::marker::{Send, Sync};
// use std::net::SocketAddr;
// use std::sync::Arc;
// use tokio::runtime::Runtime;

// // Block 区块结构体
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Block {
//     pub index: u64,
//     pub timestamp: u64,
//     pub data: String,
//     pub prev_hash: String,
//     pub hash: String,
// }

// // Blockchain 区块链结构体
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Blockchain {
//     pub blocks: Vec<Block>,
// }

// impl Blockchain {
//     pub fn new() -> Self {
//         Blockchain { blocks: vec![] }
//     }
// }

// unsafe impl Send for Blockchain {}
// unsafe impl Sync for Blockchain {}

// #[derive(Clone)]
// pub struct BlockchainServer {
//     blockchain: Arc<Blockchain>,
// }

// impl BlockchainServer {
//     pub fn new() -> Self {
//         BlockchainServer {
//             blockchain: Arc::new(Blockchain::new()),
//         }
//     }

//     async fn handle_request(&self, req: Request<Body>) -> Result<Response<Body>, String> {
//         match (req.method(), req.uri().path()) {
//             (&hyper::Method::GET, "/send_transaction") => {
//                 // 获取查询参数并解析
//                 let query = req.uri().query().ok_or("缺少查询参数")?;

//                 // 从查询参数中获取 sender、receiver 和 content
//                 // 这里需要实现解析查询参数的逻辑

//                 // 使用区块链对象发送交易
//                 // self.blockchain.send_transaction(sender, receiver, content)?;

//                 // 返回成功响应
//                 let response = Response::new(Body::from("交易已发送"));
//                 Ok(response)
//             }
//             _ => {
//                 let response = Response::builder()
//                     .status(404)
//                     .body(Body::from("未找到"))
//                     .map_err(|e| format!("创建响应失败:{}", e))?;
//                 Ok(response)
//             }
//         }
//     }

//     pub fn start(&self, addr: &str) {
//         let socket_addr: SocketAddr = addr.parse().expect("无效的地址");
//         let make_svc = make_service_fn(|_conn| {
//             let server = self.clone();
//             async { Ok::<_, hyper::Error>(service_fn(move |req| server.handle_request(req))) }
//         });

//         let server = Server::bind(&socket_addr).serve(make_svc);

//         // 使用 tokio 运行时启动服务器
//         let rt = Runtime::new().expect("创建 tokio 运行时失败");
//         rt.block_on(server).expect("服务器运行失败");
//     }
// }

// pub struct BlockchainClient {
//     base_url: String,
// }

// impl BlockchainClient {
//     pub fn new(base_url: &str) -> Self {
//         BlockchainClient {
//             base_url: base_url.to_string(),
//         }
//     }

//     pub async fn send_transaction(
//         &self,
//         sender: &str,
//         receiver: &str,
//         content: &str,
//     ) -> Result<(), String> {
//         let url = format!(
//             "{}/send_transaction?sender={}&receiver={}&content={}",
//             self.base_url, sender, receiver, content
//         );
//         let uri = Uri::try_from(url.as_str()).map_err(|e| format!("无效的URL:{}", e))?;

//         let client = Client::new();
//         let response = client
//             .get(uri)
//             .await
//             .map_err(|e| format!("发送交易请求失败:{}", e))?;

//         if response.status().is_success() {
//             Ok(())
//         } else {
//             Err(format!("发送交易失败:{}", response.status()))
//         }
//     }
// }
