# txt2audio 使用指南

`txt2audio` 是一个将英文本文件转换为音频文件的工具，通过调用 TTS (Text-to-Speech) 服务实现。支持批量处理多个文本文件。

## 前置条件

1. **TTS 服务运行中**
   - 确保 TTS 服务在 `http://localhost:5000` 运行
   - 或者使用 `--tts-url` 参数指定其他地址

2. **工作区目录结构**
   - 创建工作区目录
   - 在工作区下创建 `txt2audio_input/` 子目录
   - 将文本文件放入 `txt2audio_input/` 目录

3. **输入文件格式**
   - 纯文本文件 (.txt)
   - 每行一个英文句子
   - 空行会被自动跳过

## 工作区设置

### 1. 创建工作区目录
```bash
mkdir my_workspace
mkdir my_workspace\txt2audio_input
```

### 2. 准备输入文件
```bash
# 将文本文件复制到输入目录
copy english1.txt my_workspace\txt2audio_input\
copy english2.txt my_workspace\txt2audio_input\
```

## 基本用法

### 1. 最简单的用法
```bash
txt2audio -w my_workspace
```

### 2. 指定 TTS 服务地址
```bash
txt2audio -w my_workspace --tts-url http://192.168.1.100:5000
```

### 3. 使用自定义说话人
```bash
txt2audio -w my_workspace --speaker-wav my_speaker.wav
```

### 4. 强制覆盖已存在的文件
```bash
txt2audio -w my_workspace --force
```

## 参数说明

| 参数 | 简写 | 必需 | 默认值 | 说明 |
|------|------|------|--------|------|
| `--workspace` | `-w` | ✅ | - | 工作区目录路径 |
| `--tts-url` | - | ❌ | `http://localhost:5000` | TTS 服务地址 |
| `--speaker-wav` | - | ❌ | - | 说话人音频文件路径 |
| `--language` | - | ❌ | `en` | 语言代码 |
| `--force` | - | ❌ | `false` | 强制覆盖已存在的文件 |

## 输出文件

### 工作区结构
```
my_workspace/
├── txt2audio_input/          # 输入目录
│   ├── english1.txt
│   ├── english2.txt
│   └── ...
└── txt2audio_output/         # 输出目录（自动创建）
    ├── audio/                # 音频文件目录
    │   ├── Hello, how are you today.wav
    │   ├── This is a test sentence.wav
    │   ├── I hope this works correctly.wav
    │   ├── Thank you for using our service.wav
    │   └── ...
    ├── english1_audio_data.json
    ├── english2_audio_data.json
    └── ...
```

### 音频文件
- 格式：WAV (22050 Hz, 单声道)
- 位置：`txt2audio_output/audio/` 目录下
- 命名：`{英文内容}.wav`（特殊字符会被替换为下划线，双点号会被修复）
- 示例：`audio/Hello, how are you today.wav`, `audio/This is a test sentence.wav`

### JSON 元数据文件
- 文件名：`{输入文件名}_audio_data.json`
- 包含所有文本行和对应音频文件的信息

## 使用示例

### 示例 1：处理单个学习材料
```bash
# 1. 创建工作区
mkdir learning_workspace
mkdir learning_workspace\txt2audio_input

# 2. 准备输入文件 learning_material.txt
# 内容：
# Hello, welcome to English learning.
# Today we will learn basic vocabulary.
# Let's start with common greetings.

# 3. 运行处理
txt2audio -w learning_workspace

# 输出：
# learning_workspace/txt2audio_output/audio/Hello, welcome to English learning.wav
# learning_workspace/txt2audio_output/audio/Today we will learn basic vocabulary.wav  
# learning_workspace/txt2audio_output/audio/Let's start with common greetings.wav
# learning_workspace/txt2audio_output/learning_material_audio_data.json
```

### 示例 2：批量处理多个文件
```bash
# 1. 创建工作区
mkdir batch_workspace
mkdir batch_workspace\txt2audio_input

# 2. 复制多个文本文件到输入目录
copy *.txt batch_workspace\txt2audio_input\

# 3. 批量处理
txt2audio -w batch_workspace

# 输出：每个文件都会生成对应的音频文件和JSON元数据
```

### 示例 3：使用自定义 TTS 服务
```bash
txt2audio -w my_workspace --tts-url http://my-tts-server:8080
```

## JSON 输出格式

```json
{
  "entries": [
    {
      "text": "Hello, welcome to English learning.",
      "audio_file": "D:/workspace/txt2audio_output/audio/Hello, welcome to English learning.wav",
      "line_number": 1
    },
    {
      "text": "Today we will learn basic vocabulary.",
      "audio_file": "D:/workspace/txt2audio_output/audio/Today we will learn basic vocabulary.wav", 
      "line_number": 2
    }
  ],
  "total_count": 3,
  "output_directory": "txt2audio_output",
  "input_file": "txt2audio_input/learning_material.txt"
}
```

## 错误处理

### 常见错误及解决方案

1. **TTS 服务连接失败**
   ```
   Error: Failed to send request to TTS service
   ```
   - 检查 TTS 服务是否运行
   - 验证服务地址是否正确
   - 检查网络连接

2. **输入文件不存在**
   ```
   Error: Input file does not exist: english.txt
   ```
   - 检查文件路径是否正确
   - 确认文件存在

3. **输出目录权限问题**
   ```
   Error: Failed to create output directory
   ```
   - 检查输出目录的写入权限
   - 尝试使用其他目录

4. **TTS 服务返回错误**
   ```
   Error: TTS service error: Missing text
   ```
   - 检查输入文本是否为空
   - 验证 TTS 服务配置

## 性能优化建议

1. **批量处理**
   - 一次性处理多个句子比单独处理更高效
   - 避免频繁启动/停止 TTS 服务

2. **网络优化**
   - 使用本地 TTS 服务减少网络延迟
   - 考虑使用更快的网络连接

3. **存储优化**
   - 定期清理不需要的音频文件
   - 使用 SSD 存储提高 I/O 性能

## 故障排除

### 检查 TTS 服务状态
```bash
curl -X POST http://localhost:5000/speak \
  -H "Content-Type: application/json" \
  -d '{"text": "test", "output": "test.wav"}'
```

### 查看详细错误信息
程序会输出详细的错误信息，包括：
- 处理进度
- 成功/失败的统计
- 具体的错误原因

### 日志分析
程序运行时会显示：
- 📝 找到的文本行数
- 🎙️ 正在转换的文本
- ✅ 成功处理的统计
- ❌ 失败的原因
