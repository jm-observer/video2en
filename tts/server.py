import os
import glob
import torchaudio
import torch
import numpy as np
from flask import Flask, request, jsonify, send_file
from TTS.tts.configs.xtts_config import XttsConfig
from TTS.tts.models.xtts import Xtts

app = Flask(__name__)

# 模型路径（挂载进容器，例如 /models/XTTS-v2）
MODEL_DIR = os.environ.get("TTS_MODEL_DIR", "/models")

print(f"🔊 Loading XTTS-v2 model from: {MODEL_DIR}")

# 打印模型目录文件
print("📂 Files in model directory:")
for f in glob.glob(os.path.join(MODEL_DIR, "*")):
    print("  -", os.path.basename(f))

# 加载配置和模型
config = XttsConfig()
config.load_json(os.path.join(MODEL_DIR, "config.json"))
model = Xtts.init_from_config(config)
model.load_checkpoint(config, checkpoint_dir=MODEL_DIR, eval=True)
model.cuda()

@app.route("/speak", methods=["POST"])
def speak():
    try:
        data = request.get_json(force=True)
        text = data.get("text")
        output_path = data.get("output", "out.wav")
        language = data.get("language", "en")
        speaker_wav = data.get("speaker_wav", os.path.join(MODEL_DIR, "en_sample.wav"))

        if not text:
            return jsonify({"error": "Missing text"}), 400

        print(f"📝 Text: {text}")
        print(f"🎙️ Speaker wav: {speaker_wav}")
        print(f"💾 Output: {output_path}")

        outputs = model.synthesize(
            text,
            config,
            speaker_wav=speaker_wav,
            gpt_cond_len=3,
            language=language,
        )

        # 使用torchaudio保存音频文件
        wav_data = outputs["wav"]
        
        # 处理不同类型的音频数据
        if isinstance(wav_data, np.ndarray):
            # 将numpy数组转换为torch张量
            wav_tensor = torch.from_numpy(wav_data).float()
        elif isinstance(wav_data, torch.Tensor):
            wav_tensor = wav_data
        else:
            raise ValueError(f"Unexpected audio data type: {type(wav_data)}")
        
        # 确保音频数据格式正确
        if wav_tensor.dim() == 1:
            wav_tensor = wav_tensor.unsqueeze(0)  # 添加channel维度
        elif wav_tensor.dim() == 2 and wav_tensor.shape[0] > wav_tensor.shape[1]:
            wav_tensor = wav_tensor.transpose(0, 1)  # 转置为 (channels, samples)
        
        # 确保数据在正确的范围内 [-1, 1]
        if wav_tensor.max() > 1.0 or wav_tensor.min() < -1.0:
            wav_tensor = torch.clamp(wav_tensor, -1.0, 1.0)
        
        # 保存为WAV文件
        torchaudio.save(output_path, wav_tensor, 22050)
        return send_file(output_path, mimetype="audio/wav")

    except Exception as e:
        import traceback
        print("❌ Error in /speak:", e)
        traceback.print_exc()
        return jsonify({"error": str(e)}), 500

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=5000)
