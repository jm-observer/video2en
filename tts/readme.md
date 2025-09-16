


docker build -t coqui-tts .

docker run --gpus all -p 5000:5000 -v D:\models\tts\XTTS-v2:/models coqui-tts

curl.exe -X POST http://localhost:5000/speak `
  -H "Content-Type: application/json" `
  -d "{""text"": ""This is an American English voice generated with Coqui TTS.""}" `
  -o out.wav




# 模型下载地址：https://huggingface.co/coqui/XTTS-v2/tree/main
model.pth
config.json
speakers_xtts.pth
vocab.json
mel_stats.pth
dvae.pth