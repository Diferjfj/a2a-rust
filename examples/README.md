# A2A Rust 服务器与 Python 客户端示例

本目录包含示例实现，演示 Rust A2A 服务器和使用官方 a2a-python SDK 的 Python 客户端之间的互操作性。

## 文件说明

- `rust_server.rs` - 使用 a2a-rust 库实现的 Rust 简单回显服务器
- `python_client.py` - 使用官方 a2a-python SDK 与 Rust 服务器通信的 Python 客户端
- `README.md` - 本文件

## 前置条件

### Rust 服务器
- 已安装 Rust 1.70+
- a2a-rust 库（本项目）

### Python 客户端
- Python 3.8+
- 必需的包：`a2a-sdk`

安装 Python 依赖：
```bash
pip install a2a-sdk
```

## 运行示例

### 步骤 1：启动 Rust 服务器

在项目根目录下：

```bash
cargo run --example rust_server
```

您应该看到类似以下的输出：
```
🚀 Starting A2A Echo Server on http://127.0.0.1:8080
📋 Agent Card available at: http://127.0.0.1:8080/.well-known/agent.json
🔌 JSON-RPC endpoint at: http://127.0.0.1:8080/rpc
✨ Server is ready to accept connections!
```

### 步骤 2：运行 Python 客户端

在另一个终端中，从 `examples` 目录运行：

```bash
python python_client.py
```

您应该看到类似以下的输出：
```
🚀 A2A Python Client Example (using a2a-python)
============================================================
🔗 Connecting to Rust server at http://localhost:8080...
✅ Connected to agent: Echo Server
📝 Description: A simple echo server implemented in Rust
🌐 Server URL: http://localhost:8080
🔧 Preferred Transport: JSONRPC

📤 Test 1: Sending simple text message...
📨 Message: agent - 2 parts
   Part 1 (text): Echo from Rust server: Hello from Python a2a-client!

📤 Test 2: Sending multi-part message...
📨 Message: agent - 3 parts
   Part 1 (text): Echo from Rust server: This is a test with multiple parts:
   Part 2 (data): {'test': True, 'client': 'Python a2a-sdk'}
   Part 3 (text): Echo from Rust server: End of message

📤 Test 3: Sending message with task ID...
📨 Message: agent - 2 parts
   Part 1 (text): Echo from Rust server: Message with task context

✅ All tests completed successfully!
🎯 The Rust server and Python client are fully compatible!
```

## 示例演示内容

### Rust 服务器

Rust 服务器实现了一个简单的回显功能：

1. **Agent Card**：提供服务器的基本信息和能力
2. **JSON-RPC 端点**：处理 A2A 协议的 JSON-RPC 请求
3. **消息处理**：接收用户消息并回显内容
4. **多部分支持**：支持文本、数据等多种消息部分类型

**主要特性：**
- 使用 `A2AServerBuilder` 构建服务器
- 实现了 `RequestHandler` trait
- 支持 JSON-RPC 传输协议
- 提供完整的 Agent Card 信息

### Python 客户端

Python 客户端使用官方的 a2a-python SDK：

1. **ClientFactory**：自动连接到服务器并协商传输协议
2. **事件消费**：异步处理服务器响应和事件
3. **多类型消息**：支持文本、数据等多种消息部分
4. **上下文管理**：支持上下文 ID 和任务 ID

**主要特性：**
- 使用 `ClientFactory.connect()` 自动连接
- 实现事件消费者模式
- 支持流式响应
- 完整的错误处理

## 通信协议兼容性

本示例验证了以下兼容性：

### ✅ 已验证的兼容性

1. **Agent Card 获取**
   - Rust 服务器提供标准的 Agent Card
   - Python 客户端正确解析和使用

2. **JSON-RPC 通信**
   - 请求格式完全兼容
   - 响应解析正确
   - 错误处理机制一致

3. **消息格式**
   - TextPart 序列化/反序列化
   - DataPart 序列化/反序列化
   - 消息元数据传递

4. **传输协议协商**
   - 客户端自动选择最佳传输协议
   - 服务器能力声明正确

### 🔧 技术细节

**序列化格式：**
- Rust 使用 `serde_json` 进行 JSON 序列化
- Python 使用 `pydantic` 进行模型验证
- 双方都遵循 A2A 规范的 JSON 格式

**错误处理：**
- 标准 JSON-RPC 错误码
- A2A 特定错误类型
- 连接和超时处理

## 故障排除

### 常见问题

1. **连接被拒绝**
   ```
   ❌ Error: Connection refused
   ```
   **解决方案**：确保 Rust 服务器正在运行
   ```bash
   cargo run --example rust_server
   ```

2. **Python 包缺失**
   ```
   ❌ Missing a2a-python package: No module named 'a2a'
   ```
   **解决方案**：安装 a2a-sdk
   ```bash
   pip install a2a-sdk
   ```

3. **端口占用**
   ```
   Error: Address already in use (os error 98)
   ```
   **解决方案**：更改端口或停止占用端口的其他进程

### 调试技巧

1. **启用详细日志**
   ```bash
   RUST_LOG=debug cargo run --example rust_server
   ```

2. **检查网络连接**
   ```bash
   curl http://localhost:8080/.well-known/agent.json
   ```

3. **测试 JSON-RPC 端点**
   ```bash
   curl -X POST http://localhost:8080/rpc \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"message/send","params":{"message":{"role":"user","parts":[{"kind":"text","text":"test"}]}},"id":1}'
   ```

## 扩展示例

### 添加新功能

1. **自定义消息类型**：在服务器中添加对 FilePart 的支持
2. **流式响应**：实现服务器发送事件 (SSE)
3. **认证**：添加 API 密钥或 OAuth 认证
4. **任务管理**：实现完整的任务生命周期管理

### 性能测试

使用 `wrk` 或 `ab` 进行负载测试：
```bash
wrk -t12 -c400 -d30s http://localhost:8080/rpc
```

## 贡献

欢迎提交问题和改进建议！请确保：

1. 代码遵循 Rust 和 Python 的最佳实践
2. 添加适当的测试
3. 更新文档

## 许可证

本示例遵循与主项目相同的许可证。
