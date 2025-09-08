# 基础镜像：PyTorch GPU 版，带 CUDA 12.1 + cuDNN 8
FROM pytorch/pytorch:2.3.1-cuda12.1-cudnn8-runtime

# 设置工作目录
WORKDIR /app

# 安装必要工具
RUN apt-get update && apt-get install -y \
    git curl ffmpeg libsndfile1 && \
    rm -rf /var/lib/apt/lists/*

# 升级 pip
RUN pip install --upgrade pip

# 安装 Coqui TTS 和 Flask
RUN pip install TTS==0.22.0 flask==2.3.3

# 🔒 固定 transformers 版本，避免 generate() 报错
RUN pip install transformers==4.41.2

# 可选：锁定 torch/torchaudio 版本（和 pytorch 镜像一致）
RUN pip install torch==2.3.1 torchaudio==2.3.1

# 复制服务端脚本
COPY server.py /app/server.py

# 暴露 API 端口
EXPOSE 5000

# 运行服务
CMD ["python", "server.py"]
