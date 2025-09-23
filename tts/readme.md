# Coqui TTS 服务

## 快速开始

### 1. 构建镜像
```bash
docker build -t coqui-tts .
```

### 2. 启动服务
```bash
# 基本启动
docker run --gpus all -p 5000:5000 -v D:\workspace\models\tts\XTTS-v2:/models coqui-tts

# 后台运行并自动重启
docker run -d --name coqui-tts-service --restart unless-stopped --gpus all -p 5000:5000 -v "D:\workspace\models\tts\XTTS-v2:/models" coqui-tts
```

### 3. 测试服务
```bash
# 基本测试（使用默认说话人）
curl -X POST http://localhost:5000/speak \
  -H "Content-Type: application/json" \
  -d "{\"text\": \"This is an American English voice generated with Coqui TTS.\"}" \
  -o test.wav

# 指定说话人文件测试
curl -X POST http://localhost:5000/speak \
  -H "Content-Type: application/json" \
  -d "{\"text\": \"Hello world\", \"speaker_wav\": \"en_sample.wav\"}" \
  -o test_speaker.wav
```

## 管理命令

### 查看服务状态
```bash
docker ps --filter "name=coqui-tts-service"
```

### 查看日志
```bash
docker logs coqui-tts-service
```

### 停止服务
```bash
docker stop coqui-tts-service
```

### 删除容器
```bash
docker rm coqui-tts-service
```

### 重启服务
```bash
docker restart coqui-tts-service
```

## 开机自启动

使用 `--restart unless-stopped` 参数可以让容器在系统重启后自动启动：

```bash
docker run -d \
    --name coqui-tts-service \
    --restart unless-stopped \
    --gpus all \
    -p 5000:5000 \
    -v "D:\workspace\models\tts\XTTS-v2:/models" \
    coqui-tts
```

## 模型文件

下载地址：https://huggingface.co/coqui/XTTS-v2/tree/main

### 必需文件：
- model.pth
- config.json
- speakers_xtts.pth
- vocab.json
- mel_stats.pth
- dvae.pth

### 说话人文件：
- `en_sample.wav` - 默认英语说话人样本（需要手动添加到模型目录）
- 其他 `.wav` 文件 - 自定义说话人样本

**注意：** 说话人文件需要放在模型目录 `/models/` 中，服务会自动查找可用的 `.wav` 文件作为说话人样本。

## 说话人样本下载

可以使用提供的脚本下载说话人样本：

```bash
# 下载英语说话人样本
python download_tts_wav.py
```

下载的样本会保存在 `speaker_samples/` 目录中，然后可以复制到模型目录使用。

## API 参数

### POST /speak

**请求参数：**
- `text` (必需): 要转换的文本
- `language` (可选): 语言代码，默认为 "en"
- `speaker_wav` (可选): 说话人文件名，默认为 "en_sample.wav"

**响应：**
- 成功：返回 WAV 音频文件
- 失败：返回 JSON 错误信息

**示例：**
```json
{
  "text": "Hello world",
  "language": "en",
  "speaker_wav": "en_sample.wav"
}
```

## 故障排除

### 检查 GPU 支持
```bash
docker run --rm --gpus all nvidia/cuda:11.0-base nvidia-smi
```

### 检查端口占用
```bash
netstat -an | findstr :5000
```

### 查看详细错误
```bash
docker logs -f coqui-tts-service
```