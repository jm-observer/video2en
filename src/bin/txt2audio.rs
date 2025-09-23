use anyhow::{anyhow, Context, Result};
use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tokio::fs as async_fs;

#[derive(Parser, Debug)]
#[command(
    name = "txt2audio",
    about = "Convert English text to audio using TTS service",
    version,
    long_about = "A Rust CLI tool that reads English text files from a workspace directory and converts each line to audio using a TTS service. \
                  Supports batch processing of multiple text files. \
                  Uses a workspace directory with fixed subdirectories: txt2audio_input/, txt2audio_output/"
)]
struct Args {
    /// Workspace directory containing txt2audio_input/ and txt2audio_output/ subdirectories
    #[arg(short, long, value_name = "WORKSPACE_DIR")]
    workspace: PathBuf,

    /// TTS service URL
    #[arg(long, value_name = "URL", default_value = "http://localhost:5000")]
    tts_url: String,

    /// Speaker audio file path for TTS
    #[arg(long, value_name = "SPEAKER_WAV")]
    speaker_wav: Option<PathBuf>,

    /// Language for TTS
    #[arg(long, value_name = "LANG", default_value = "en")]
    language: String,

    /// Force overwrite existing files
    #[arg(long)]
    force: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct AudioEntry {
    text: String,
    female_audio: String,
    male_audio: String,
    line_number: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct AudioData {
    entries: Vec<AudioEntry>,
    total_count: usize,
    output_directory: String,
    input_file: String,
}

struct Txt2Audio {
    args: Args,
}

impl Txt2Audio {
    fn new(args: Args) -> Self {
        Self { args }
    }

    fn get_workspace_paths(&self) -> Result<(PathBuf, PathBuf)> {
        let workspace = &self.args.workspace;
        if !workspace.exists() {
            return Err(anyhow!("Workspace directory does not exist: {}", workspace.display()));
        }
        
        let input_dir = workspace.join("txt2audio_input");
        let output_dir = workspace.join("txt2audio_output");

        if !input_dir.exists() {
            return Err(anyhow!("Input directory does not exist: {}", input_dir.display()));
        }

        // 创建输出目录
        fs::create_dir_all(&output_dir).context("Failed to create output directory")?;
        
        Ok((input_dir, output_dir))
    }

    fn get_input_files(&self) -> Result<Vec<PathBuf>> {
        let (input_dir, _) = self.get_workspace_paths()?;
        let mut text_files = Vec::new();
        
        for entry in fs::read_dir(&input_dir)
            .context(format!("Failed to read input directory: {}", input_dir.display()))? 
        {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if ext == "txt" {
                        text_files.push(path);
                    }
                }
            }
        }

        if text_files.is_empty() {
            return Err(anyhow!("No .txt files found in input directory: {}", input_dir.display()));
        }

        Ok(text_files)
    }

    fn handle_output_directory(&self, output_dir: &Path) -> Result<()> {
        if !output_dir.exists() {
            fs::create_dir_all(output_dir).context("Failed to create output directory")?;
            println!("📁 创建输出目录: {}", output_dir.display());
            return Ok(());
        }

        let mut entries = fs::read_dir(output_dir).context("Failed to read output directory")?;
        if entries.next().is_none() {
            println!("📁 输出目录为空，直接使用: {}", output_dir.display());
            return Ok(());
        }

        println!("📁 输出目录不为空，正在重命名: {}", output_dir.display());
        let mut backup_name = output_dir.with_file_name(format!("{}_backup", 
            output_dir.file_name().unwrap_or_default().to_string_lossy()));
        let mut counter = 1;
        
        while backup_name.exists() {
            backup_name = output_dir.with_file_name(format!("{}_backup_{}", 
                output_dir.file_name().unwrap_or_default().to_string_lossy(), 
                counter
            ));
            counter += 1;
        }

        fs::rename(output_dir, &backup_name).context("Failed to rename output directory")?;
        println!("📁 已重命名为: {}", backup_name.display());
        
        fs::create_dir_all(output_dir).context("Failed to create new output directory")?;
        println!("📁 创建新的输出目录: {}", output_dir.display());
        
        Ok(())
    }
}

struct TtsClient {
    client: Client,
    base_url: String,
    language: String,
}

impl TtsClient {
    fn new(base_url: String, language: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            language,
        }
    }

    async fn text_to_speech(&self, text: &str, output_path: &Path, speaker_wav: Option<&str>) -> Result<()> {
        let url = format!("{}/speak", self.base_url);
        
        let mut request_body = serde_json::json!({
            "text": text,
            "language": self.language
        });

        // 如果指定了speaker_wav，添加到请求中
        if let Some(speaker_path) = speaker_wav {
            request_body["speaker_wav"] = serde_json::Value::String(speaker_path.to_string());
        }

        println!("🎙️ Converting: {}", text);
        println!("💾 Output: {}", output_path.display());

        let response = self.client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .context("Failed to send request to TTS service")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!("TTS service error: {}", error_text));
        }

        // 检查响应内容类型
        let content_type = response.headers()
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown");
        
        println!("📡 Response content-type: {}", content_type);

        // 保存音频文件
        let audio_data = response.bytes().await.context("Failed to get audio data")?;
        
        if audio_data.is_empty() {
            return Err(anyhow!("TTS service returned empty audio data"));
        }
        
        println!("📊 Audio data size: {} bytes", audio_data.len());
        
        async_fs::write(output_path, audio_data)
            .await
            .context("Failed to write audio file")?;

        Ok(())
    }
}

impl Txt2Audio {
    async fn process_text_file(&self, input_file: &Path, output_dir: &Path) -> Result<()> {
        // 读取文本文件
        let content = async_fs::read_to_string(input_file)
            .await
            .context("Failed to read input file")?;

        let lines: Vec<&str> = content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();

        if lines.is_empty() {
            println!("⚠️ No valid text lines found in: {}", input_file.display());
            return Ok(());
        }

        println!("📝 Found {} text lines to process", lines.len());

        // 创建TTS客户端
        let tts_client = TtsClient::new(
            self.args.tts_url.clone(),
            self.args.language.clone(),
        );

        // 获取输入文件名（不含扩展名）用于输出文件命名
        let input_stem = input_file
            .file_stem()
            .ok_or_else(|| anyhow!("Invalid input filename"))?
            .to_string_lossy()
            .to_string();

        let mut audio_entries = Vec::new();

        // 处理每一行文本
        for (index, line) in lines.iter().enumerate() {
            let line_number = index + 1;
            // 将英文内容作为文件名，替换特殊字符和中文
            let safe_filename = line
                .chars()
                .map(|c| match c {
                    '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                    c if c.is_ascii() => c,
                    _ => '_', // 将所有非ASCII字符（包括中文）替换为下划线
                })
                .collect::<String>()
                .trim_matches('_') // 移除首尾的下划线
                .to_string()
                .replace("..", ".") // 替换双点号为单点号
                .trim_end_matches('.') // 移除末尾的点号
                .to_string();
            
            // 如果文件名为空或太短，使用行号作为文件名
            let audio_filename = if safe_filename.len() < 3 {
                format!("line_{:03}.wav", line_number)
            } else {
                format!("{}.wav", safe_filename)
            };
            
            // 创建 audio 子目录
            let audio_dir = output_dir.join("audio");
            async_fs::create_dir_all(&audio_dir).await
                .context("Failed to create audio directory")?;
            
            // 检查女性声音文件是否已存在
            let female_filename = audio_filename.replace(".wav", "_female.wav");
            let female_path = audio_dir.join(&female_filename);
            let female_file_path = female_path.to_string_lossy().replace('\\', "/").replace("//", "/");
            
            // 检查男性声音文件是否已存在
            let male_filename = audio_filename.replace(".wav", "_male.wav");
            let male_path = audio_dir.join(&male_filename);
            let male_file_path = male_path.to_string_lossy().replace('\\', "/").replace("//", "/");

            // 如果两个文件都存在且不强制覆盖，则跳过
            if female_path.exists() && male_path.exists() && !self.args.force {
                println!("⏭️ Skipping line {} (both files exist): {}", line_number, line);
                audio_entries.push(AudioEntry {
                    text: line.to_string(),
                    female_audio: female_file_path,
                    male_audio: male_file_path,
                    line_number,
                });
                continue;
            }

            // 调用TTS服务 - 女性声音
            tts_client.text_to_speech(line, &female_path, None).await?;

            // 调用TTS服务 - 男性声音
            tts_client.text_to_speech(line, &male_path, Some("1320-122617-0037.wav")).await?;

            // 只有当至少一个音频生成成功时才添加到结果中
            audio_entries.push(AudioEntry {
                text: line.to_string(),
                female_audio: female_file_path,
                male_audio: male_file_path,
                line_number,
            });

            // 添加小延迟避免过度请求
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        // 生成JSON输出文件
        let json_data = AudioData {
            entries: audio_entries,
            total_count: lines.len(),
            output_directory: output_dir.to_string_lossy().to_string(),
            input_file: input_file.to_string_lossy().to_string(),
        };

        let json_filename = format!("{}_audio_data.json", input_stem);
        let json_path = output_dir.join(&json_filename);

        let json_content = serde_json::to_string_pretty(&json_data)
            .context("Failed to serialize JSON data")?;

        async_fs::write(&json_path, json_content)
            .await
            .context("Failed to write JSON file")?;

        println!("📄 JSON data file: {}", json_path.display());
        Ok(())
    }

    async fn run(&self) -> Result<()> {
        let input_files = self.get_input_files()?;
        println!("📁 找到 {} 个输入文件", input_files.len());
        
        let (_, output_dir) = self.get_workspace_paths()?;
        self.handle_output_directory(&output_dir)?;

        let mut total_processed = 0;
        let mut total_failed = 0;

        for (index, input_file) in input_files.iter().enumerate() {
            println!("\n📄 处理文件 {}/{}: {}", 
                index + 1, input_files.len(), input_file.display());
            
            match self.process_text_file(input_file, &output_dir).await {
                Ok(()) => {
                    total_processed += 1;
                    println!("✅ 文件 {} 处理完成!", 
                        input_file.file_name().unwrap_or_default().to_string_lossy());
                }
                Err(e) => {
                    total_failed += 1;
                    println!("❌ 文件 {} 处理失败: {}", 
                        input_file.file_name().unwrap_or_default().to_string_lossy(), e);
                }
            }
        }

        println!("\n🎉 所有文件处理完成！");
        println!("📊 统计信息:");
        println!("   - 总文件数: {}", input_files.len());
        println!("   - 成功处理: {}", total_processed);
        println!("   - 处理失败: {}", total_failed);
        println!("📁 输出目录: {}", output_dir.display());

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("🎵 TTS Text-to-Audio Converter");
    println!("📁 Workspace: {}", args.workspace.display());
    println!("🌐 TTS service: {}", args.tts_url);
    if let Some(ref speaker) = args.speaker_wav {
        println!("🎙️ Speaker file: {}", speaker.display());
    }
    println!("🗣️ Language: {}", args.language);

    let processor = Txt2Audio::new(args);
    processor.run().await
}
