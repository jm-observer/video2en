# Video2En 演示脚本
# 这个脚本演示了如何使用 video2en 工具

Write-Host "🎬 Video2En 演示脚本" -ForegroundColor Green
Write-Host "================================" -ForegroundColor Green

# 检查程序是否存在
if (Test-Path ".\target\release\video2en.exe") {
    Write-Host "✅ 找到 video2en.exe" -ForegroundColor Green
} else {
    Write-Host "❌ 未找到 video2en.exe，请先运行 cargo build --release" -ForegroundColor Red
    exit 1
}

# 检查 ffmpeg 是否可用
try {
    $ffmpegVersion = ffmpeg -version 2>&1 | Select-Object -First 1
    Write-Host "✅ FFmpeg 可用: $ffmpegVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ FFmpeg 不可用，请先安装 FFmpeg" -ForegroundColor Red
    Write-Host "   可以使用: winget install ffmpeg" -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "📋 使用方法示例:" -ForegroundColor Cyan
Write-Host "1. 基本用法:" -ForegroundColor White
Write-Host "   .\target\release\video2en.exe -i <视频文件> -m <模型文件>" -ForegroundColor Gray
Write-Host ""
Write-Host "2. 指定输出目录:" -ForegroundColor White
Write-Host "   .\target\release\video2en.exe -i <视频文件> -m <模型文件> -o <输出前缀>" -ForegroundColor Gray
Write-Host ""
Write-Host "3. 强制覆盖:" -ForegroundColor White
Write-Host "   .\target\release\video2en.exe -i <视频文件> -m <模型文件> --force" -ForegroundColor Gray
Write-Host ""
Write-Host "4. 指定语言和线程数:" -ForegroundColor White
Write-Host "   .\target\release\video2en.exe -i <视频文件> -m <模型文件> --language en --threads 8" -ForegroundColor Gray

Write-Host ""
Write-Host "📁 输出文件:" -ForegroundColor Cyan
Write-Host "   - <前缀>.all.srt  (全量字幕)" -ForegroundColor Gray
Write-Host "   - <前缀>.en.srt   (英文字幕)" -ForegroundColor Gray
Write-Host "   - <前缀>.en.txt   (英文文本)" -ForegroundColor Gray

Write-Host ""
Write-Host "⚠️  注意: 当前版本为演示模式，使用模拟数据" -ForegroundColor Yellow
Write-Host "    要使用真实的 Whisper 功能，需要安装 libclang" -ForegroundColor Yellow
Write-Host "    详细说明请查看 README.md" -ForegroundColor Yellow

Write-Host ""
Write-Host "🎯 下一步:" -ForegroundColor Cyan
Write-Host "1. 准备一个视频/音频文件" -ForegroundColor White
Write-Host "2. 下载 Whisper GGML 模型 (如 ggml-small.bin)" -ForegroundColor White
Write-Host "3. 运行转换命令" -ForegroundColor White
