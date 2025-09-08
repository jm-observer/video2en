# Video2En 安装指南

## 系统要求

- **操作系统**: Windows 10/11 (x64)
- **内存**: 建议 8GB+ RAM
- **存储**: 至少 2GB 可用空间
- **网络**: 需要下载模型文件

## 安装步骤

### 1. 安装 Rust

1. 访问 [https://rustup.rs/](https://rustup.rs/)
2. 下载并运行 `rustup-init.exe`
3. 选择默认安装选项
4. 重启命令提示符或 PowerShell

验证安装：
```powershell
rustc --version
cargo --version
```

### 2. 安装 FFmpeg

#### 方法 1: 使用 winget (推荐)
```powershell
winget install ffmpeg
```

#### 方法 2: 使用 Chocolatey
```powershell
choco install ffmpeg
```

#### 方法 3: 手动安装
1. 访问 [https://ffmpeg.org/download.html](https://ffmpeg.org/download.html)
2. 下载 Windows 版本
3. 解压到 `C:\ffmpeg`
4. 将 `C:\ffmpeg\bin` 添加到 PATH 环境变量

验证安装：
```powershell
ffmpeg -version
```

### 3. 安装 libclang (用于 Whisper 功能)

#### 方法 1: 使用 winget
```powershell
winget install LLVM.LLVM
```

#### 方法 2: 使用 Chocolatey
```powershell
choco install llvm
```

#### 方法 3: 手动安装
1. 访问 [https://github.com/llvm/llvm-project/releases](https://github.com/llvm/llvm-project/releases)
2. 下载最新的 Windows 版本
3. 安装并添加到 PATH

设置环境变量：
```powershell
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
```

### 4. 构建项目

```powershell
# 克隆项目
git clone <your-repo-url>
cd video2en

# 构建发布版本
cargo build --release

# 验证构建
.\target\release\video2en.exe --help
```

### 5. 下载 Whisper 模型

```powershell
# 创建模型目录
mkdir models
cd models

# 下载模型文件
# 小模型 (推荐用于测试)
curl -L -o ggml-small.bin https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin

# 基础模型
curl -L -o ggml-base.bin https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin

# 大模型 (更准确但更慢)
curl -L -o ggml-large.bin https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large.bin
```

## 故障排除

### 常见问题

#### 1. "ffmpeg not found in PATH"
- 确保 FFmpeg 已正确安装
- 检查 PATH 环境变量
- 重启命令提示符

#### 2. "Unable to find libclang"
- 安装 LLVM/Clang
- 设置 `LIBCLANG_PATH` 环境变量
- 重启命令提示符

#### 3. 编译错误
- 确保 Rust 版本 >= 1.70
- 运行 `rustup update`
- 清理并重新构建：`cargo clean && cargo build --release`

#### 4. 内存不足
- 使用较小的模型文件
- 减少线程数
- 关闭其他应用程序

### 环境变量设置

```powershell
# 设置 LIBCLANG_PATH
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"

# 添加到系统环境变量 (永久)
[Environment]::SetEnvironmentVariable("LIBCLANG_PATH", "C:\Program Files\LLVM\bin", "User")
```

### 验证安装

运行以下命令验证所有组件：

```powershell
# 检查 Rust
rustc --version

# 检查 FFmpeg
ffmpeg -version

# 检查 LLVM
clang --version

# 检查项目
.\target\release\video2en.exe --help
```

## 下一步

安装完成后，请查看：
- [README.md](README.md) - 使用说明
- [test_demo.ps1](test_demo.ps1) - 演示脚本

## 支持

如果遇到问题，请：
1. 检查本文档的故障排除部分
2. 查看项目 Issues
3. 提供详细的错误信息和系统信息
