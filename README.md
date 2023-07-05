# chatgpt-agent

## 简介

本应用并未记录/保存任何请求体中的内容。

这是一个用来代理 `chatgpt api` 的服务。请参考 `chatgpt-api` 文档，编写请求，该服务会透传给 `chatgpt server`，并将返回值原样返回。

为每一个独立的用户申请 `chatgpt token` 后，由管理员维护进数据库，并向用户颁发唯一的 `token` 用来请求 `agent`，需要在 `header` 中携带 `Authorization Bearer your_token`。

在获取到 `chatgpt-api` 返回值后，会取出其中的 `token` 用量并计入数据库，用于计算调用接口耗费的资源。

会对数据库表中有配置价格的模型进行操作，选用其它模型会被拒绝。

请先找管理员申请 token。

## 流程图

```mermaid
sequenceDiagram
    用户->>agent: POST /chat
    activate agent
    agent ->> agent: 校验token
    agent ->> 用户: 401 校验未通过
    agent ->> chatgpt-server: POST api 接口
    activate chatgpt-server
    deactivate agent
    chatgpt-server ->> agent: 失败信息
    agent ->> 用户: 失败信息
    chatgpt-server ->> agent: 成功信息
    deactivate chatgpt-server
    agent ->> agent: 保存 usage 中信息
    agent ->> 用户: gpt 响应信息
```

## 用法
```
POST /chat
Authorization: Bearer your_token
Content-Type: application-json
{
    "model": "gpt-3.5-turbo-0301",
    "messages": [
        {
            "role": "user",
            "content": "你好"
        },
        {
                "role": "assistant",
                "content": "你好！有什么我可以帮助你的吗？"
        },
        {
            "role": "user",
            "content": "测试数据测试数据"
        },
        {
                "role": "assistant",
                "content": "我是一个AI语言模型，被称为语言模型助手，可以回答你的问题、进行对话、提供帮助等。"
        },
        {
            "role": "user",
            "content": "ai 有人权吗"
        }
    ],
    "temperature": 0.7
}
```

## config

| 名称              | 描述                                   | 默认值                                     | 示例                                                 |
| ----------------- | -------------------------------------- | ------------------------------------------ | ---------------------------------------------------- |
| BIND_ADDRESS      | 绑定的地址                             | 0.0.0.0                                    |                                                      |
| PORT              | 服务启动端口号                         | 8080                                       |                                                      |
| DATABASE_URL      | 数据库连接值，有它会忽略之后数据库设置 |                                            | postgres://username:password@127.0.0.1:5433/database |
| DATABASE_USERNAME | 数据库用户名                           | postgres                                   |                                                      |
| DATABASE_PASSWORD | 数据库密码                             | postgres                                   |                                                      |
| DATABASE_ADDRESS  | 数据库地址                             | 127.0.0.1:5432                             |                                                      |
| OPENAI_URL        | openai url                             | https://api.openai.com/v1/chat/completions |                                                      |
| OPENAI_PROXY      | http代理地址                           |                                            | http://localhost:7890                                |

