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

### 3. 使用默认男女音频（自动生成两个版本）
```bash
txt2audio -w my_workspace
```
程序会自动生成男声和女声两个版本的音频文件：
- 男声：使用 `1320-122617-0037.wav`（默认）
- 女声：使用 `en_sample.wav`（默认）

### 4. 自定义男女音频说话人
```bash
txt2audio -w my_workspace --male-speaker-wav 1320-122617-0037.wav --female-speaker-wav en_sample.wav
```

### 5. 强制覆盖已存在的文件
```bash
txt2audio -w my_workspace --force
```

## 参数说明

| 参数 | 简写 | 必需 | 默认值 | 说明 |
|------|------|------|--------|------|
| `--workspace` | `-w` | ✅ | - | 工作区目录路径 |
| `--tts-url` | - | ❌ | `http://localhost:5000` | TTS 服务地址 |
| `--male-speaker-wav` | - | ❌ | `1320-122617-0037.wav` | 男声说话人音频文件名 |
| `--female-speaker-wav` | - | ❌ | `en_sample.wav` | 女声说话人音频文件名 |
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
    │   ├── Hello,_how_are_you_today_female.wav    # 女声版本
    │   ├── Hello,_how_are_you_today_male.wav      # 男声版本
    │   ├── This_is_a_test_sentence_female.wav
    │   ├── This_is_a_test_sentence_male.wav
    │   ├── I_hope_this_works_correctly_female.wav
    │   ├── I_hope_this_works_correctly_male.wav
    │   └── ...
    ├── english1_audio_data.json
    ├── english2_audio_data.json
    └── ...
```

### 音频文件
- 格式：WAV (22050 Hz, 单声道)
- 位置：`txt2audio_output/audio/` 目录下
- 命名：`{英文内容}_female.wav` 和 `{英文内容}_male.wav`（特殊字符会被替换为下划线）
- 每行文本生成两个音频文件：女声版本和男声版本

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

# 输出（每行文本生成男女两个版本）：
# learning_workspace/txt2audio_output/audio/Hello,_welcome_to_English_learning_female.wav
# learning_workspace/txt2audio_output/audio/Hello,_welcome_to_English_learning_male.wav
# learning_workspace/txt2audio_output/audio/Today_we_will_learn_basic_vocabulary_female.wav
# learning_workspace/txt2audio_output/audio/Today_we_will_learn_basic_vocabulary_male.wav
# learning_workspace/txt2audio_output/audio/Let's_start_with_common_greetings_female.wav
# learning_workspace/txt2audio_output/audio/Let's_start_with_common_greetings_male.wav
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

### 示例 3：使用自定义男女音频说话人
```bash
# 使用不同的音频样本文件
txt2audio -w my_workspace \
  --male-speaker-wav 1320-122617-0037.wav \
  --female-speaker-wav en_sample.wav
```

### 示例 4：使用自定义 TTS 服务
```bash
txt2audio -w my_workspace --tts-url http://my-tts-server:8080
```

## JSON 输出格式

```json
{
  "entries": [
    {
      "text": "Hello, welcome to English learning.",
      "female_audio": "txt2audio_output/audio/Hello,_welcome_to_English_learning_female.wav",
      "male_audio": "txt2audio_output/audio/Hello,_welcome_to_English_learning_male.wav",
      "line_number": 1
    },
    {
      "text": "Today we will learn basic vocabulary.",
      "female_audio": "txt2audio_output/audio/Today_we_will_learn_basic_vocabulary_female.wav",
      "male_audio": "txt2audio_output/audio/Today_we_will_learn_basic_vocabulary_male.wav",
      "line_number": 2
    }
  ],
  "total_count": 2,
  "output_directory": "txt2audio_output",
  "input_file": "txt2audio_input/learning_material.txt"
}
```

## 可选音频说话人列表

音频说话人文件位于 TTS 服务的 `workspace/models/tts/XTTS-v2/` 目录下。

### 默认配置
- **男声默认**：`1320-122617-0037.wav`
- **女声默认**：`en_sample.wav`

### 如何使用其他音频说话人
1. 查看 TTS 服务的 `workspace/models/tts/XTTS-v2/` 目录
2. 选择合适的 WAV 音频文件
3. 使用参数指定：
   ```bash
   txt2audio -w my_workspace \
     --male-speaker-wav <音频文件名.wav> \
     --female-speaker-wav <音频文件名.wav>
   ```

### 音频说话人选择建议
- 选择发音清晰的音频样本
- 音频长度建议 5-10 秒
- 音频质量越好，生成的语音质量越高
- 可以使用不同语言的音频样本实现跨语言语音克隆

### 常见音频说话人示例
以下是可能在 `workspace/models/tts/XTTS-v2/` 目录中找到的音频文件：

- `en_sample.wav` - 英文女声样本（默认女声）
- `1320-122617-0037.wav` - 英文男声样本（默认男声）
- 其他自定义音频文件...

**注意**：具体可用的音频文件取决于您的 TTS 服务配置。请查看实际目录以获取完整列表。

### 添加自定义音频说话人
您也可以添加自己的音频样本：
1. 准备一个 5-10 秒的清晰音频文件（WAV 格式）
2. 将文件放入 `workspace/models/tts/XTTS-v2/` 目录
3. 使用文件名作为参数：
   ```bash
   txt2audio -w my_workspace --male-speaker-wav my_custom_voice.wav
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
