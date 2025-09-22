use anyhow::{anyhow, Context, Result};
use clap::Parser;
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use regex::Regex;
use std::{fs, path::{Path, PathBuf}, process::Command};
use video2en::youdao::YoudaoTranslator;

#[derive(Parser, Debug)]
#[command(
    name = "video2en",
    about = "Extract English subtitles from video/audio files using Whisper",
    version,
    long_about = "A Rust CLI tool that extracts audio from video/audio files, \
                  transcribes them using Whisper, and filters for English content. \
                  Supports GPU acceleration (CUDA/OpenCL) for faster processing. \
                  Outputs three files: full SRT, English-only SRT, and English-only TXT. \
                  Uses a workspace directory with fixed subdirectories: input/, models/, output/"
)]
struct Args {
    /// Workspace directory containing input/, models/, and output/ subdirectories
    #[arg(short, long, value_name = "WORKSPACE_DIR")]
    workspace: PathBuf,

    /// Model filename (default: ggml-large.bin)
    #[arg(long, value_name = "MODEL_NAME")]
    model_name: Option<String>,

    /// Recognition language
    #[arg(long, value_name = "auto|en|zh", default_value = "auto")]
    language: String,

    /// Number of threads for recognition
    #[arg(long, value_name = "N")]
    threads: Option<usize>,

    /// Use GPU acceleration (CUDA/Vulkan)
    #[arg(long)]
    gpu: bool,

    /// GPU device ID (default: 0)
    #[arg(long, value_name = "ID", default_value = "0")]
    gpu_device: u32,

    /// Force overwrite existing output files
    #[arg(long)]
    force: bool,

    /// Enable translation
    #[arg(long)]
    translate: bool,

}

#[derive(Debug, Clone)]
struct Segment {
    start_ms: u32,
    end_ms: u32,
    text: String,
    translation: Option<String>,
}


struct Video2En {
    args: Args,
    temp_dir: tempfile::TempDir,
    language_detector: LanguageDetector,
}

impl Video2En {
    fn new(args: Args) -> Result<Self> {
        let temp_dir = tempfile::tempdir().context("Failed to create temporary directory")?;
        
        let language_detector = LanguageDetectorBuilder::from_languages(&[
            Language::English,
            Language::Chinese,
        ])
        .build();

        Ok(Self {
            args,
            temp_dir,
            language_detector,
        })
    }

    /// 获取workspace中的固定子文件夹路径
    fn get_workspace_paths(&self) -> Result<(PathBuf, PathBuf, PathBuf)> {
        let workspace = &self.args.workspace;
        
        // 确保workspace目录存在
        if !workspace.exists() {
            return Err(anyhow!("Workspace directory does not exist: {}", workspace.display()));
        }
        
        let input_dir = workspace.join("video2en_input");
        let models_dir = workspace.join("models");
        let output_dir = workspace.join("video2en_output");
        
        // 检查input和models目录是否存在
        if !input_dir.exists() {
            return Err(anyhow!("Input directory does not exist: {}", input_dir.display()));
        }
        if !models_dir.exists() {
            return Err(anyhow!("Models directory does not exist: {}", models_dir.display()));
        }
        
        // 只创建output目录（如果不存在）
        fs::create_dir_all(&output_dir).context("Failed to create output directory")?;
        
        Ok((input_dir, models_dir, output_dir))
    }

    /// 获取输入文件路径列表（从workspace/input/目录中查找）
    fn get_input_files(&self) -> Result<Vec<PathBuf>> {
        let (input_dir, _, _) = self.get_workspace_paths()?;
        
        // 查找input目录中的视频/音频文件
        let mut video_files = Vec::new();
        for entry in fs::read_dir(&input_dir).context(format!("Failed to read input directory: {}", input_dir.display()))? {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if matches!(ext.as_str(), "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | 
                               "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a") {
                        video_files.push(path);
                    }
                }
            }
        }
        
        if video_files.is_empty() {
            return Err(anyhow!("No video/audio files found in input directory: {}", input_dir.display()));
        }
        
        Ok(video_files)
    }

    /// 获取模型文件路径（从workspace/models/目录中查找）
    fn get_model_file(&self, model_name: Option<String>) -> Result<PathBuf> {
        let (_, models_dir, _) = self.get_workspace_paths()?;
        
        // 确定要查找的模型文件名
        let target_model_name = model_name.unwrap_or_else(|| "ggml-large.bin".to_string());
        let target_path = models_dir.join(&target_model_name);
        
        // 检查指定的模型文件是否存在
        if target_path.exists() && target_path.is_file() {
            return Ok(target_path);
        }
        
        // 如果指定的文件不存在，查找models目录中的所有.bin文件
        let mut model_files = Vec::new();
        for entry in fs::read_dir(&models_dir).context(format!("Failed to read models directory: {}", models_dir.display()))? {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "bin" {
                        model_files.push(path);
                    }
                }
            }
        }
        
        if model_files.is_empty() {
            return Err(anyhow!("No .bin model files found in models directory: {}", models_dir.display()));
        }
        
        if model_files.len() > 1 {
            return Err(anyhow!("Multiple model files found in models directory. Please specify model name with --model-name or keep only one file: {:?}", model_files));
        }
        
        Ok(model_files[0].clone())
    }

    async fn run(&self) -> Result<()> {
        // Check ffmpeg availability
        self.check_ffmpeg()?;

        // 获取所有输入文件
        let input_files = self.get_input_files()?;
        
        println!("📁 找到 {} 个输入文件", input_files.len());

        let (_, _, output_dir) = self.get_workspace_paths()?;
        
        // 检查并处理输出目录
        self.handle_output_directory(&output_dir)?;
        
        // 循环处理每个输入文件
        for (index, input_file) in input_files.iter().enumerate() {
            println!("\n🎬 处理文件 {}/{}: {}", index + 1, input_files.len(), input_file.display());
            
            // 获取当前文件的输出前缀
            // let output_prefix = self.get_output_prefix_for_file(input_file)?;
            
            // Extract audio
            let audio_path = self.extract_audio_for_file(input_file, &output_dir)?;
            
            // Transcribe with whisper-cli.exe
            let segments = self.transcribe(&audio_path, &output_dir)?;
            
            // 分析和统计英文内容
            self.write_outputs(&segments, &audio_path).await?;
            
            println!("✅ 文件 {} 处理完成!", input_file.file_name().unwrap_or_default().to_string_lossy());
            println!("📁 生成的文件:");
            println!("   - {} (音频文件)", audio_path.display());
        }

        println!("\n🎉 所有文件处理完成！共处理了 {} 个文件", input_files.len());
        Ok(())
    }

    fn check_ffmpeg(&self) -> Result<()> {
        which::which("ffmpeg").map_err(|_| {
            anyhow!(
                "ffmpeg not found in PATH. Please install ffmpeg:\n\
                 Windows: Download from https://ffmpeg.org/download.html\n\
                 Or use chocolatey: choco install ffmpeg\n\
                 Or use winget: winget install ffmpeg"
            )
        })?;
        Ok(())
    }

    fn handle_output_directory(&self, output_dir: &Path) -> Result<()> {
        if !output_dir.exists() {
            // 输出目录不存在，创建它
            fs::create_dir_all(output_dir).context("Failed to create output directory")?;
            println!("📁 创建输出目录: {}", output_dir.display());
            return Ok(());
        }

        // 检查输出目录是否为空
        let mut entries = fs::read_dir(output_dir).context("Failed to read output directory")?;
        if entries.next().is_none() {
            // 目录为空，直接使用
            println!("📁 输出目录为空，直接使用: {}", output_dir.display());
            return Ok(());
        }

        // 目录不为空，需要重命名
        println!("📁 输出目录不为空，正在重命名: {}", output_dir.display());
        
        let mut backup_name = output_dir.with_file_name(format!("{}_backup", output_dir.file_name().unwrap_or_default().to_string_lossy()));
        let mut counter = 1;
        
        // 处理多次重命名的情况
        while backup_name.exists() {
            backup_name = output_dir.with_file_name(format!("{}_backup_{}", 
                output_dir.file_name().unwrap_or_default().to_string_lossy(), 
                counter
            ));
            counter += 1;
        }
        
        // 重命名原目录
        fs::rename(output_dir, &backup_name).context("Failed to rename output directory")?;
        println!("📁 已重命名为: {}", backup_name.display());
        
        // 创建新的输出目录
        fs::create_dir_all(output_dir).context("Failed to create new output directory")?;
        println!("📁 创建新的输出目录: {}", output_dir.display());
        
        Ok(())
    }


    fn extract_audio(&self, output_prefix: &Path) -> Result<PathBuf> {
        let input_files = self.get_input_files()?;
        let input_path = &input_files[0]; // 使用第一个文件
        
        // 获取输入文件名（不含扩展名）
        let input_stem = input_path
            .file_stem()
            .ok_or_else(|| anyhow!("Invalid input filename"))?
            .to_string_lossy()
            .to_string();
        
        // 创建音频文件路径：输出目录 + 输入文件名 + .wav
        let audio_path = output_prefix.join(format!("{}.wav", input_stem));

        // 确保音频文件的父目录存在
        if let Some(parent) = audio_path.parent() {
            fs::create_dir_all(parent).context("Failed to create audio output directory")?;
        }

        println!("🎵 Extracting audio from: {}", input_path.display());
        println!("💾 Audio will be saved to: {}", audio_path.display());

        let status = Command::new("ffmpeg")
            .args([
                "-y",                    // Overwrite output
                "-i", input_path.to_str().unwrap(),
                "-vn",                   // No video
                "-ac", "1",             // Mono
                "-ar", "16000",         // 16kHz sample rate
                "-f", "wav",            // WAV format
                audio_path.to_str().unwrap(),
            ])
            .status()
            .context("Failed to execute ffmpeg")?;

        if !status.success() {
            return Err(anyhow!("ffmpeg failed with exit code: {}", status));
        }

        Ok(audio_path)
    }

    fn extract_audio_for_file(&self, input_path: &Path, output_prefix: &Path) -> Result<PathBuf> {
        // 获取输入文件名（不含扩展名）
        let input_stem = input_path
            .file_stem()
            .ok_or_else(|| anyhow!("Invalid input filename"))?
            .to_string_lossy()
            .to_string();
        
        // 创建音频文件路径：输出目录 + 输入文件名 + .wav
        let audio_path = output_prefix.join(format!("{}.wav", input_stem));

        // 确保音频文件的父目录存在
        if let Some(parent) = audio_path.parent() {
            fs::create_dir_all(parent).context("Failed to create audio output directory")?;
        }

        println!("🎵 Extracting audio from: {}", input_path.display());
        println!("💾 Audio will be saved to: {}", audio_path.display());

        let status = Command::new("ffmpeg")
            .args([
                "-y",                    // Overwrite output
                "-i", input_path.to_str().unwrap(),
                "-vn",                   // No video
                "-ac", "1",             // Mono
                "-ar", "16000",         // 16kHz sample rate
                "-f", "wav",            // WAV format
                audio_path.to_str().unwrap(),
            ])
            .status()
            .context("Failed to execute ffmpeg")?;

        if !status.success() {
            return Err(anyhow!("ffmpeg failed with exit code: {}", status));
        }

        Ok(audio_path)
    }



    fn transcribe(&self, audio_path: &Path, output_prefix: &Path) -> Result<Vec<Segment>> {
        println!("🤖 Transcribing audio using whisper-cli.exe...");
        
        // 检查 whisper-cli.exe 是否可用
        self.check_whisper_cli()?;
        
        // 获取输出目录和文件名
        // let output_prefix = self.get_output_prefix()?;
        // let output_dir = &output_prefix;
        let output_name = audio_path.file_stem().unwrap_or_default().to_string_lossy().to_string();
        let txt_output = output_prefix.join(format!("{}_raw", output_name));

        // println!("📁 Output directory: {}", output_dir.display());
        
        // 构建 whisper-cli 命令 - 使用指定的参数格式
        let model = self.get_model_file(self.args.model_name.clone())?;
        let mut cmd = Command::new("whisper-cli.exe");
        cmd.arg("-m").arg(model)
           .arg("-f").arg(audio_path.to_str().unwrap())
        //    .arg("-l").arg("en")  // 固定为英文
           .arg("-tr")           // 翻译
           .arg("-bs").arg("8")  // batch size
           .arg("-bo").arg("1")  // best of
           .arg("-t").arg("8")   // threads
           .arg("-otxt")         // 输出文本格式
           .arg("-of").arg(txt_output.clone());
        
        println!("🎯 Running whisper-cli with command: {:?}", cmd);
        
        // 执行命令
        let output = cmd.output()
            .context("Failed to execute whisper-cli.exe")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(anyhow!("whisper-cli failed:\nSTDERR: {}\nSTDOUT: {}", stderr, stdout));
        }
        
        // 读取生成的文本文件（whisper-cli会自动添加.txt扩展名）
        let txt_output = output_prefix.join(format!("{}_raw.txt", output_name));
        let text_content = fs::read_to_string(&txt_output)
            .context(format!("Failed to read generated text file: {}", txt_output.display()))?;
        
        // 解析文本内容为segments（每行作为一个segment）
        let segments = self.parse_text_to_segments(&text_content);
        
        // 保留whisper-cli生成的中间文本文件
        println!("📄 保留中间文本文件: {}", txt_output.display());
        
        println!("✅ Transcribed {} text segments", segments.len());
        Ok(segments)
    }
    
    fn check_whisper_cli(&self) -> Result<()> {
        which::which("whisper-cli.exe").map_err(|_| {
            anyhow!(
                "whisper-cli.exe not found in PATH. Please install whisper-cli:\n\
                 Download from: https://github.com/ggerganov/whisper.cpp/releases\n\
                 Or build from source: https://github.com/ggerganov/whisper.cpp"
            )
        })?;
        Ok(())
    }
    
    fn parse_text_to_segments(&self, text_content: &str) -> Vec<Segment> {
        let mut segments = Vec::new();
        let mut start_time = 0u32;
        
        for line in text_content.lines() {
            let text = line.trim();
            if !text.is_empty() {
                // 简单的时间分配：每行假设持续3秒
                let end_time = start_time + 3000;
                
                segments.push(Segment {
                    start_ms: start_time,
                    end_ms: end_time,
                    text: text.to_string(),
                    translation: None,
                });
                
                start_time = end_time;
            }
        }
        
        segments
    }
    
    fn parse_srt(&self, srt_content: &str) -> Result<Vec<Segment>> {
        let mut segments = Vec::new();
        let re = Regex::new(r"(\d+)\n(\d{2}:\d{2}:\d{2},\d{3}) --> (\d{2}:\d{2}:\d{2},\d{3})\n(.*?)(?=\n\d+\n|\n*$)").unwrap();
        
        for cap in re.captures_iter(srt_content) {
            let start_time = self.parse_timestamp(&cap[2])?;
            let end_time = self.parse_timestamp(&cap[3])?;
            let text = cap[4].trim().to_string();
            
            if !text.is_empty() {
                segments.push(Segment {
                    start_ms: start_time,
                    end_ms: end_time,
                    text,
                    translation: None,
                });
            }
        }
        
        Ok(segments)
    }
    
    fn parse_timestamp(&self, timestamp: &str) -> Result<u32> {
        let parts: Vec<&str> = timestamp.split(':').collect();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid timestamp format: {}", timestamp));
        }
        
        let hours: u32 = parts[0].parse()?;
        let minutes: u32 = parts[1].parse()?;
        let seconds_parts: Vec<&str> = parts[2].split(',').collect();
        if seconds_parts.len() != 2 {
            return Err(anyhow!("Invalid seconds format: {}", parts[2]));
        }
        
        let seconds: u32 = seconds_parts[0].parse()?;
        let milliseconds: u32 = seconds_parts[1].parse()?;
        
        Ok(hours * 3600000 + minutes * 60000 + seconds * 1000 + milliseconds)
    }
    

    // fn get_output_prefix(&self) -> Result<PathBuf> {
    //     let (_, _, output_dir) = self.get_workspace_paths()?;
        
    //     // 获取输入文件名作为输出前缀
    //     let input_files = self.get_input_files()?;
    //     let input_file = &input_files[0]; // 使用第一个文件
    //     let input_stem = input_file
    //         .file_stem()
    //         .ok_or_else(|| anyhow!("Invalid input filename"))?
    //         .to_string_lossy()
    //         .to_string();
        
    //     let output_prefix = output_dir.join(input_stem);
        
    //     // 确保输出目录存在
    //     if let Some(parent) = output_prefix.parent() {
    //         fs::create_dir_all(parent).context("Failed to create output directory")?;
    //     }
        
    //     Ok(output_prefix)
    // }


    async fn write_outputs(&self, segments: &[Segment], audio_path: &Path) -> Result<()> {
        // 过滤英文segments
        let english_segments: Vec<&Segment> = segments
            .iter()
            .filter(|segment| self.is_english(&segment.text))
            .collect();

        // 去重：使用HashSet来存储唯一的英文文本
        use std::collections::HashSet;
        let mut unique_english_texts = HashSet::new();
        let mut deduplicated_segments = Vec::new();
        
        for segment in &english_segments {
            let normalized_text = self.normalize_text(&segment.text);
            if unique_english_texts.insert(normalized_text.clone()) {
                deduplicated_segments.push((*segment).clone());
            }
        }

        // 统计信息
        let total_segments = segments.len();
        let english_segments_count = english_segments.len();
        let unique_english_count = deduplicated_segments.len();
        let duplicate_count = english_segments_count - unique_english_count;
        let non_english_segments_count = total_segments - english_segments_count;

        println!("📊 统计结果:");
        println!("   - 总段落数: {}", total_segments);
        println!("   - 英文段落数: {}", english_segments_count);
        println!("   - 去重后英文段落数: {}", unique_english_count);
        println!("   - 重复英文段落数: {}", duplicate_count);
        println!("   - 非英文段落数: {}", non_english_segments_count);
        
        if total_segments > 0 {
            let english_percentage = (english_segments_count as f64 / total_segments as f64) * 100.0;
            let unique_percentage = (unique_english_count as f64 / total_segments as f64) * 100.0;
            println!("   - 英文比例: {:.1}%", english_percentage);
            println!("   - 去重后英文比例: {:.1}%", unique_percentage);
        }

        // 保存去重后的英文内容到文件
        if !deduplicated_segments.is_empty() {
            // 如果启用了翻译功能，则翻译去重后的英文内容
            if self.args.translate {
                self.translate_segments(&mut deduplicated_segments).await?;
            }
            
            let output_file = audio_path.with_file_name(
                audio_path.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string() + ".txt"
            );
            self.save_unique_english(&deduplicated_segments.iter().collect::<Vec<_>>(), &output_file)?;
            
            // 显示去重后的英文内容预览
            println!("📝 去重后英文内容预览 (前10段):");
            for (i, segment) in deduplicated_segments.iter().take(10).enumerate() {
                println!("   {}. {}", i + 1, segment.text);
                if let Some(ref translation) = segment.translation {
                    println!("      中文: {}", translation);
                }
            }
            if deduplicated_segments.len() > 10 {
                println!("   ... 还有 {} 段去重后的英文内容", deduplicated_segments.len() - 10);
            }
            
            println!("💾 去重后的英文内容已保存到: {}", output_file.display());
        }

        Ok(())
    }

    fn write_srt(&self, segments: &[&Segment], output_path: &Path, description: &str) -> Result<()> {
        if output_path.exists() && !self.args.force {
            println!("[skip] {} SRT already exists: {}", description, output_path.display());
            return Ok(());
        }

        println!("📝 Writing {} SRT: {}", description, output_path.display());
        
        let mut content = String::new();
        for (i, segment) in segments.iter().enumerate() {
            let start_time = self.format_timestamp(segment.start_ms);
            let end_time = self.format_timestamp(segment.end_ms);
            
            content.push_str(&format!("{}\n", i + 1));
            content.push_str(&format!("{} --> {}\n", start_time, end_time));
            content.push_str(&format!("{}\n\n", segment.text));
        }

        fs::write(output_path, content)
            .context(format!("Failed to write SRT file: {}", output_path.display()))?;

        Ok(())
    }

    fn write_txt(&self, segments: &[&Segment], output_path: &Path) -> Result<()> {
        if output_path.exists() && !self.args.force {
            println!("[skip] English TXT already exists: {}", output_path.display());
            return Ok(());
        }

        println!("📄 Writing English TXT: {}", output_path.display());
        
        let content: String = segments
            .iter()
            .map(|segment| segment.text.clone())
            .collect::<Vec<_>>()
            .join("\n\n");

        fs::write(output_path, content)
            .context(format!("Failed to write TXT file: {}", output_path.display()))?;

        Ok(())
    }

    fn is_english(&self, text: &str) -> bool {
        // Clean text for analysis
        let cleaned = self.clean_text(text);
        
        if cleaned.is_empty() {
            return false;
        }

        // Method 1: ASCII letter ratio check
        let ascii_letters: usize = cleaned
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .count();
        
        let total_chars = cleaned.chars().count();
        if total_chars > 0 {
            let ascii_ratio = ascii_letters as f64 / total_chars as f64;
            if ascii_ratio >= 0.6 {
                return true;
            }
        }

        // Method 2: Language detection fallback
        if let Some(language) = self.language_detector.detect_language_of(&cleaned) {
            return language == Language::English;
        }

        false
    }

    fn clean_text(&self, text: &str) -> String {
        // Remove common non-text symbols and normalize
        let re = Regex::new(r"[^\p{L}\p{N}\s]").unwrap();
        re.replace_all(text, " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn normalize_text(&self, text: &str) -> String {
        // 标准化文本用于去重比较
        text.trim()
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic() || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    async fn translate_segments(&self, segments: &mut Vec<Segment>) -> Result<()> {
        let translator = YoudaoTranslator;
        
        println!("🌐 正在翻译英文内容...");
        
        let total_count = segments.len();
        for (i, segment) in segments.iter_mut().enumerate() {
            print!("\r🔄 翻译进度: {}/{}", i + 1, total_count);
            std::io::Write::flush(&mut std::io::stdout()).ok();
            
            match translator.translate(&segment.text).await {
                Ok(word_info) => {
                    if let Some(fanyi) = &word_info.fanyi {
                        segment.translation = Some(fanyi.tran.clone());
                    } else {
                        segment.translation = Some("未找到翻译".to_string());
                    }
                }
                Err(e) => {
                    println!("\n⚠️ 翻译失败: {} - {}", segment.text, e);
                    segment.translation = Some("翻译失败".to_string());
                }
            }
            
            // 添加小延迟避免API限制
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        println!("\n✅ 翻译完成!");
        Ok(())
    }


    fn save_unique_english(&self, segments: &Vec<&Segment>, output_path: &Path) -> Result<()> {
        if output_path.exists() && !self.args.force {
            println!("[skip] 去重英文文件已存在: {}", output_path.display());
            return Ok(());
        }

        println!("📄 保存去重后的英文内容到: {}", output_path.display());
        
        let mut content = String::new();
        // content.push_str("# 去重后的英文内容 (中英文对照)\n");
        // content.push_str(&format!("# 总计 {} 段唯一英文内容\n\n", segments.len()));
        
        for (_i, segment) in segments.iter().enumerate() {
            content.push_str(&format!("{}\n", segment.text));
            // if let Some(ref translation) = segment.translation {
            //     content.push_str(&format!("   中文: {}\n", translation));
            // }
            // content.push_str("\n");
        }

        fs::write(output_path, content)
            .context(format!("Failed to write unique English file: {}", output_path.display()))?;

        Ok(())
    }

    fn format_timestamp(&self, ms: u32) -> String {
        let seconds = ms / 1000;
        let milliseconds = ms % 1000;
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        
        format!("{:02}:{:02}:{:02},{:03}", hours, minutes, secs, milliseconds)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // 验证workspace目录存在
    if !args.workspace.exists() {
        return Err(anyhow!("Workspace directory does not exist: {}", args.workspace.display()));
    }

    let processor = Video2En::new(args)?;
    
    // 验证输入文件和模型文件存在（这些验证现在在get_input_files和get_model_file中进行）
    let _input_files = processor.get_input_files()?;
    let _model_file = processor.get_model_file(processor.args.model_name.clone())?;
    
    processor.run().await
}


#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_translation() {
        let translator = YoudaoTranslator;
        
        let test_text = "It's peaceful".to_string();
        println!("📝 测试文本: {}", test_text);
        
        match translator.translate(&test_text).await {
            Ok(word_info) => {
                println!("✅ 翻译成功!");
                println!("   英文: {}", test_text);
                
                // 从fanyi字段获取翻译
                if let Some(fanyi) = &word_info.fanyi {
                    println!("   中文: {}", fanyi.tran);
                } else {
                    println!("   中文: 未找到翻译");
                }
            }
            Err(e) => {
                println!("❌ 翻译失败: {}", e);
            }
        }
    }
}