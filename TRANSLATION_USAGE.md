# 有道翻译API使用说明

## 功能概述

video2en工具现在支持将有道翻译API集成，可以将提取的英文内容自动翻译成中文，生成中英文对照的学习材料。

## 获取有道翻译API密钥

1. 访问 [有道智云](https://ai.youdao.com/)
2. 注册账号并登录
3. 创建应用，获取 `应用ID` 和 `应用密钥`
4. 确保应用已开通"文本翻译"服务

## 使用方法

### 方法1: 环境变量（推荐）

```bash
# 设置环境变量
export YOUDAO_APP_KEY="your_app_key_here"
export YOUDAO_APP_SECRET="your_app_secret_here"

# 运行程序并启用翻译
cargo run -- -i input.mp4 -m model.bin -o output/ --translate
```

### 方法2: 命令行参数

```bash
cargo run -- -i input.mp4 -m model.bin -o output/ \
  --translate \
  --youdao-app-key "your_app_key_here" \
  --youdao-app-secret "your_app_secret_here"
```

### 方法3: Windows PowerShell

```powershell
# 设置环境变量
$env:YOUDAO_APP_KEY="your_app_key_here"
$env:YOUDAO_APP_SECRET="your_app_secret_here"

# 运行程序
cargo run -- -i input.mp4 -m model.bin -o output/ --translate
```

## 输出格式

启用翻译功能后，程序会生成包含中英文对照的文件：

```
# 去重后的英文内容 (中英文对照)
# 总计 122 段唯一英文内容

1. Let's practice speaking in English.
   中文: 让我们练习说英语。

2. I'm screwed.
   中文: 我完蛋了。

3. She's attractive.
   中文: 她很迷人。
```

## 注意事项

1. **API限制**: 有道翻译API有调用频率限制，程序会自动添加延迟避免超限
2. **网络要求**: 需要稳定的网络连接访问有道API
3. **费用**: 有道翻译API可能有费用，请查看官方定价
4. **翻译质量**: 翻译结果仅供参考，建议人工校对重要内容

## 错误处理

如果翻译过程中出现错误，程序会：
- 显示错误信息
- 将失败的翻译标记为"翻译失败"
- 继续处理其他内容
- 不会中断整个程序运行

## 示例完整命令

```bash
# 完整示例：提取音频、转录、去重、翻译
cargo run -- \
  -i "D:\videos\english_lesson.mp4" \
  -m "D:\models\ggml-medium.bin" \
  -o "D:\output\" \
  --gpu \
  --translate \
  --youdao-app-key "your_key" \
  --youdao-app-secret "your_secret"
```

## 技术实现

- 使用MD5签名算法确保API请求安全性
- 异步处理提高翻译效率
- 批量翻译减少API调用次数
- 自动重试和错误恢复机制
