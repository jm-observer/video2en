# 翻译功能测试说明

## 测试方法

我已经添加了一个测试功能来验证有道翻译API是否正常工作。

### 运行测试

```bash
# 测试翻译功能
cargo run -- --test-translation
```

### 测试内容

测试会尝试翻译以下英文短语：
- **测试文本**: "It's peaceful"
- **预期结果**: 应该返回中文翻译

### 测试输出示例

```
🧪 测试有道翻译API...
📝 测试文本: It's peaceful
✅ 翻译成功!
   英文: It's peaceful
   中文: 很平静
```

### 错误处理

如果API密钥无效或网络连接有问题，会显示错误信息：

```
🧪 测试有道翻译API...
📝 测试文本: It's peaceful
❌ 翻译失败: Youdao API error: 108
```

### 注意事项

1. **API密钥**: 需要将代码中的占位符替换为实际的有道API密钥
2. **网络连接**: 需要稳定的网络连接访问有道API
3. **API限制**: 有道API可能有调用频率限制

### 修改API密钥

在 `src/main.rs` 文件中找到以下代码：

```rust
let app_key = "your_app_key_here".to_string();
let app_secret = "your_app_secret_here".to_string();
```

将 `"your_app_key_here"` 和 `"your_app_secret_here"` 替换为实际的有道翻译API密钥。

### 完整测试流程

1. 确保网络连接正常
2. 设置正确的API密钥
3. 运行测试命令
4. 查看翻译结果
5. 如果成功，可以继续使用 `--translate` 参数进行实际翻译
