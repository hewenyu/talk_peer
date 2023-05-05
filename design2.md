
graph LR
A[客户端] -->|请求| B[服务端]
B -->|响应| A
B --> C[Matrix协议]
C --> D[去中心化网络]
C --> E[端到端加密]
C --> F[可扩展性]

subgraph 客户端
    A1[桌面端] --> A
    A2[移动端] --> A
    A3[Web端] --> A
end

subgraph 服务端
    B1[用户数据管理] --> B
    B2[API接口] --> B
    B3[消息处理] --> B
end