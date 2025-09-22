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

### 工作区目录结构

程序使用统一的工作区目录，包含以下固定子文件夹：

```
workspace/
├── video2en_input/     # 放置输入视频/音频文件（必须预先创建）
├── models/             # 放置Whisper模型文件(.bin)（必须预先创建）
└── video2en_output/    # 输出文件目录（程序会自动创建）
```

**注意**：
- `video2en_input/` 和 `models/` 目录必须预先创建，程序不会自动创建这些目录
- `video2en_output/` 目录如果不存在，程序会自动创建
- `video2en_input/` 目录中可以放置多个视频/音频文件，程序会循环处理所有找到的文件
- `models/` 目录中可以放置多个 `.bin` 模型文件，程序会使用指定的模型文件（默认：`ggml-large.bin`）
- 输出文件直接放在 `video2en_output/` 目录下，不会创建子目录

### 基本用法

```powershell
# 使用工作区目录（推荐方式）
.\target\release\video2en.exe -w D:\my_workspace

# 启用翻译功能
.\target\release\video2en.exe -w D:\my_workspace --translate

# 指定语言和线程数
.\target\release\video2en.exe -w D:\my_workspace --language en --threads 8
```

### 参数说明

- `-w, --workspace <WORKSPACE_DIR>`: 工作区目录路径（必需）
- `--model-name <MODEL_NAME>`: 模型文件名，默认为 `ggml-large.bin`
- `--language <auto|en|zh>`: 识别语言，默认 `auto`（自动检测）
- `--threads <N>`: 识别线程数，默认使用所有可用 CPU 核心
- `--force`: 强制覆盖已存在的输出文件

### 使用示例

假设您有一个工作区目录 `D:\video_processing`，结构如下：

```
D:\video_processing\
├── video2en_input\      # 必须预先创建
│   ├── 视频1.mp4
│   ├── 视频2.mp4
│   └── 音频1.wav
├── models\              # 必须预先创建
│   └── ggml-large-v3.bin
└── video2en_output\     # 程序会自动创建
    ├── 视频1.wav
    ├── 视频1.txt
    ├── 视频2.wav
    ├── 视频2.txt
    ├── 音频1.wav
    └── 音频1.txt
```

**准备工作**：
```powershell
# 创建工作区目录结构
mkdir D:\video_processing
mkdir D:\video_processing\video2en_input
mkdir D:\video_processing\models
# video2en_output目录不需要手动创建，程序会自动创建
```

**运行命令**：
```powershell
# 使用默认模型文件名 (ggml-large.bin)
.\target\release\video2en.exe -w D:\video_processing

# 指定特定的模型文件名
.\target\release\video2en.exe -w D:\video_processing --model-name ggml-small.bin
```

程序会：
1. 检查 `video2en_input/` 和 `models/` 目录是否存在
2. 自动从 `video2en_input/` 目录找到所有视频/音频文件
3. 循环处理每个文件：
   - 提取音频到 `video2en_output/文件名.wav`
   - 使用指定的模型文件进行语音识别
   - 生成英文内容到 `video2en_output/文件名.txt`
4. 显示处理进度和完成统计

### 输出文件

程序会为每个输入文件生成以下文件：

1. `<文件名>.wav` - 提取的音频文件
2. `<文件名>.txt` - 去重后的英文内容文本文件

### 处理流程

1. **扫描输入文件**：自动扫描 `video2en_input/` 目录中的所有视频/音频文件
2. **循环处理**：对每个文件执行以下步骤：
   - **音频提取**：使用 FFmpeg 从视频文件中提取音频
   - **语音识别**：使用 Whisper 模型将音频转换为文本
   - **语言过滤**：自动识别并过滤出英文内容
   - **去重处理**：去除重复的英文内容
   - **文件输出**：生成音频文件和英文文本文件
3. **进度显示**：显示当前处理进度和总体统计信息



## 性能优化建议

1. **模型选择**：
   - `ggml-small.bin`: 快速，适合实时处理
   - `ggml-base.bin`: 平衡速度和准确性
   - `ggml-large.bin`: 最高准确性，但处理较慢（默认）

2. **批量处理**：
   - 将多个视频文件放在 `video2en_input/` 目录中，程序会自动循环处理
   - 每个文件都会生成独立的输出文件，避免冲突

3. **线程数设置**：
   - 根据 CPU 核心数调整 `--threads` 参数
   - 通常设置为 CPU 核心数的 1-2 倍

4. **语言设置**：
   - 如果确定是纯英文内容，使用 `--language en` 可提高速度
   - 中英混杂内容使用 `--language auto`

## 故障排除

### 常见错误

1. **"ffmpeg not found in PATH"**
   - 确保已正确安装 FFmpeg
   - 检查 PATH 环境变量是否包含 FFmpeg 的 bin 目录

2. **"Input directory does not exist"**
   - 确保 `video2en_input/` 目录已创建
   - 检查工作区目录路径是否正确

3. **"Models directory does not exist"**
   - 确保 `models/` 目录已创建
   - 检查工作区目录路径是否正确

4. **"No video/audio files found in input directory"**
   - 确保 `video2en_input/` 目录中有视频或音频文件
   - 支持的文件格式：mp4, avi, mkv, mov, wmv, flv, webm, mp3, wav, flac, aac, ogg, m4a

5. **"No .bin model files found in models directory"**
   - 确保 `models/` 目录中有 `.bin` 模型文件
   - 使用 `--model-name` 参数指定特定的模型文件

6. **"Multiple model files found"**
   - 使用 `--model-name` 参数指定要使用的模型文件
   - 或者在 `models/` 目录中只保留一个模型文件

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
