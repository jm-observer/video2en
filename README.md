# Video2En - 视频到英文字幕转换工具

一个用 Rust 编写的命令行工具，可以从视频/音频文件中提取音频，使用 Whisper 进行语音识别，并过滤出英文字幕。

## 功能特性

- 🎵 支持多种视频/音频格式（mp4, mkv, mov, mp3, wav 等）
- 🤖 使用 Whisper GGML 模型进行离线语音识别
- 🌍 支持中英混杂内容，自动识别语言
- 📝 输出三种格式：全量字幕、英文字幕、英文纯文本
- ⚡ 多线程处理，支持自定义线程数

- 🔄 智能覆盖策略，避免重复处理

## 系统要求

- Windows 10/11 (x64)
- Rust 1.70+ 
- FFmpeg (需要预先安装)

## 安装步骤

### 1. 安装 Rust

访问 [https://rustup.rs/](https://rustup.rs/) 下载并安装 Rust。

### 2. 安装 FFmpeg

选择以下任一方式：

**方式 1: 使用 winget (推荐)**
```powershell
winget install ffmpeg
```

**方式 2: 使用 Chocolatey**
```powershell
choco install ffmpeg
```

**方式 3: 手动下载**
- 访问 [https://ffmpeg.org/download.html](https://ffmpeg.org/download.html)
- 下载 Windows 版本
- 解压到某个目录，并将 `bin` 目录添加到 PATH 环境变量

### 3. 下载 Whisper 模型

下载 GGML 格式的 Whisper 模型：

```powershell
# 创建模型目录
mkdir models
cd models

# 下载模型 (选择其中一个)
# 小模型 (推荐用于测试)
curl -L -o ggml-small.bin https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin

# 基础模型
curl -L -o ggml-base.bin https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin

# 大模型 (更准确但更慢)
curl -L -o ggml-large.bin https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large.bin
```

### 4. 构建项目

```powershell
# 克隆项目
git clone <your-repo-url>
cd video2en

# 构建发布版本
cargo build --release

# 可执行文件位于 target/release/video2en.exe
```

## 使用方法

### 基本用法

```powershell
# 使用默认输出前缀（与输入文件同名）
.\target\release\video2en.exe -i input.mp4 -m models\ggml-small.bin

# 指定输出前缀
.\target\release\video2en.exe -i input.mp4 -m models\ggml-small.bin -o output\my_video

# 强制覆盖已存在的文件
.\target\release\video2en.exe -i input.mp4 -m models\ggml-small.bin --force

# 指定语言和线程数
.\target\release\video2en.exe -i input.mp4 -m models\ggml-small.bin --language en --threads 8
```

### 参数说明

- `-i, --input <PATH>`: 输入视频/音频文件路径（必需）
- `-m, --model <PATH>`: Whisper GGML 模型文件路径（必需）
- `-o, --output <PREFIX>`: 输出文件前缀（可选，默认为输入文件名）
- `--language <auto|en|zh>`: 识别语言，默认 `auto`（自动检测）
- `--threads <N>`: 识别线程数，默认使用所有可用 CPU 核心

- `--force`: 强制覆盖已存在的输出文件

### 输出文件

程序会在指定前缀下生成三个文件：

1. `<prefix>.all.srt` - 全量字幕文件（包含中英文）
2. `<prefix>.en.srt` - 仅英文字幕文件
3. `<prefix>.en.txt` - 仅英文纯文本文件

## 示例

### 示例 1: 基本转换

```powershell
.\target\release\video2en.exe -i "D:\Videos\meeting.mp4" -m "D:\models\ggml-small.bin"
```

输出文件：
- `D:\Videos\meeting.all.srt`
- `D:\Videos\meeting.en.srt`
- `D:\Videos\meeting.en.txt`

### 示例 2: 指定输出目录

```powershell
.\target\release\video2en.exe -i "D:\Videos\meeting.mp4" -m "D:\models\ggml-small.bin" -o "D:\Output\meeting_output"
```

输出文件：
- `D:\Output\meeting_output.all.srt`
- `D:\Output\meeting_output.en.srt`
- `D:\Output\meeting_output.en.txt`

### 示例 3: 英文专用模式

```powershell
.\target\release\video2en.exe -i "D:\Videos\english_video.mp4" -m "D:\models\ggml-small.bin" --language en --threads 12
```



## 性能优化建议

1. **模型选择**：
   - `ggml-small.bin`: 快速，适合实时处理
   - `ggml-base.bin`: 平衡速度和准确性
   - `ggml-large.bin`: 最高准确性，但处理较慢

2. **线程数设置**：
   - 根据 CPU 核心数调整 `--threads` 参数
   - 通常设置为 CPU 核心数的 1-2 倍

3. **语言设置**：
   - 如果确定是纯英文内容，使用 `--language en` 可提高速度
   - 中英混杂内容使用 `--language auto`

## 故障排除

### 常见错误

1. **"ffmpeg not found in PATH"**
   - 确保已正确安装 FFmpeg
   - 检查 PATH 环境变量是否包含 FFmpeg 的 bin 目录

2. **"Model file does not exist"**
   - 检查模型文件路径是否正确
   - 确保模型文件已下载完成

3. **"Expected mono audio, got X channels"**
   - 这是正常情况，程序会自动将音频转换为单声道

4. **内存不足错误**
   - 尝试使用较小的模型文件
   - 减少线程数

### 性能问题

- 长视频处理时间较长是正常的
- 使用 SSD 存储可以提高 I/O 性能
- 确保有足够的 RAM（建议 8GB+）

## 技术细节

- **音频处理**: 使用 FFmpeg 将音频转换为 16kHz 单声道 WAV 格式
- **语音识别**: 基于 whisper.cpp 的 Rust 绑定
- **语言检测**: 结合 ASCII 比例检测和 lingua 库
- **字幕格式**: 标准 SRT 格式，支持毫秒级时间戳

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！

## 更新日志

### v0.1.0
- 初始版本
- 支持基本的视频到字幕转换
- 支持中英文混合内容识别
- 输出 SRT 和 TXT 格式
