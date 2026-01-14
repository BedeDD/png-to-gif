use serde::Serialize;
use std::process::{Command, Stdio};
use tauri::Emitter;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as AsyncCommand;

#[derive(Serialize, Clone)]
pub struct FfmpegInfo {
    pub installed: bool,
    pub version: Option<String>,
    pub path: Option<String>,
}

#[derive(Clone)]
pub struct ConversionConfig {
    pub input_pattern: String,
    pub start_number: usize,
    pub framerate: u32,
    pub width: u32,
    pub loop_forever: bool,
    pub output_path: String,
    pub directory: String,
    pub total_frames: usize,
}

pub fn check_ffmpeg() -> FfmpegInfo {
    // Try to find ffmpeg in PATH first
    let ffmpeg_path = which::which("ffmpeg")
        .or_else(|_| {
            // If not in PATH, check common installation locations
            let common_paths = if cfg!(target_os = "windows") {
                vec![
                    "C:\\ffmpeg\\bin\\ffmpeg.exe",
                    "C:\\Program Files\\ffmpeg\\bin\\ffmpeg.exe",
                    "C:\\Program Files (x86)\\ffmpeg\\bin\\ffmpeg.exe",
                ]
            } else if cfg!(target_os = "macos") {
                vec![
                    "/opt/homebrew/bin/ffmpeg",     // Homebrew Apple Silicon
                    "/usr/local/bin/ffmpeg",        // Homebrew Intel
                    "/usr/bin/ffmpeg",              // System
                    "/opt/local/bin/ffmpeg",        // MacPorts
                ]
            } else {
                vec![
                    "/usr/bin/ffmpeg",              // System
                    "/usr/local/bin/ffmpeg",        // Local install
                    "/snap/bin/ffmpeg",             // Snap
                ]
            };

            common_paths.into_iter()
                .find(|p| std::path::Path::new(p).exists())
                .map(|p| std::path::PathBuf::from(p))
                .ok_or_else(|| which::Error::CannotFindBinaryPath)
        });

    match ffmpeg_path {
        Ok(path) => {
            // Try to get version
            let version = Command::new(&path)
                .arg("-version")
                .output()
                .ok()
                .and_then(|output| {
                    String::from_utf8(output.stdout)
                        .ok()
                        .and_then(|s| s.lines().next().map(|l| l.to_string()))
                });

            FfmpegInfo {
                installed: true,
                version,
                path: path.to_str().map(|s| s.to_string()),
            }
        }
        Err(_) => FfmpegInfo {
            installed: false,
            version: None,
            path: None,
        },
    }
}

pub async fn convert_to_gif(
    config: ConversionConfig,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    // Find ffmpeg binary
    let ffmpeg_path = which::which("ffmpeg")
        .or_else(|_| {
            let common_paths = if cfg!(target_os = "windows") {
                vec![
                    "C:\\ffmpeg\\bin\\ffmpeg.exe",
                    "C:\\Program Files\\ffmpeg\\bin\\ffmpeg.exe",
                    "C:\\Program Files (x86)\\ffmpeg\\bin\\ffmpeg.exe",
                ]
            } else if cfg!(target_os = "macos") {
                vec![
                    "/opt/homebrew/bin/ffmpeg",
                    "/usr/local/bin/ffmpeg",
                    "/usr/bin/ffmpeg",
                    "/opt/local/bin/ffmpeg",
                ]
            } else {
                vec![
                    "/usr/bin/ffmpeg",
                    "/usr/local/bin/ffmpeg",
                    "/snap/bin/ffmpeg",
                ]
            };
            common_paths.into_iter()
                .find(|p| std::path::Path::new(p).exists())
                .map(|p| std::path::PathBuf::from(p))
                .ok_or_else(|| which::Error::CannotFindBinaryPath)
        })
        .map_err(|_| "FFmpeg not found".to_string())?;

    // Build FFmpeg command
    let mut cmd = AsyncCommand::new(ffmpeg_path);

    cmd.current_dir(&config.directory);

    // Input parameters
    cmd.arg("-framerate")
        .arg(config.framerate.to_string())
        .arg("-start_number")
        .arg(config.start_number.to_string())
        .arg("-i")
        .arg(&config.input_pattern);

    // Single-pass filter with scaling, palette generation and application
    // This preserves transparency by using split filter and reserve_transparent
    cmd.arg("-vf")
        .arg(format!(
            "scale={}:-1:flags=lanczos,split[s0][s1];[s0]palettegen=reserve_transparent=1[p];[s1][p]paletteuse=alpha_threshold=128",
            config.width
        ));

    // Disable GIF offsetting for proper transparency support
    cmd.arg("-gifflags").arg("-offsetting");

    // Loop settings
    if config.loop_forever {
        cmd.arg("-loop").arg("0");
    } else {
        cmd.arg("-loop").arg("-1");
    }

    // Output
    cmd.arg("-y") // Overwrite without asking
        .arg(&config.output_path);

    // Debug: Write command to log file
    let log_path = std::path::Path::new(&config.directory).join("ffmpeg_command.log");
    let cmd_str = format!(
        "ffmpeg -framerate {} -start_number {} -i \"{}\" -vf \"scale={}:-1:flags=lanczos,split[s0][s1];[s0]palettegen=reserve_transparent=1[p];[s1][p]paletteuse=alpha_threshold=128\" -gifflags -offsetting -loop {} -y \"{}\"\n\nDirectory: {}\nTimestamp: {:?}",
        config.framerate,
        config.start_number,
        config.input_pattern,
        config.width,
        if config.loop_forever { "0" } else { "-1" },
        config.output_path,
        config.directory,
        std::time::SystemTime::now()
    );
    let _ = std::fs::write(&log_path, cmd_str);

    // Stderr for progress
    cmd.stderr(Stdio::piped());
    cmd.stdout(Stdio::null());

    let mut child = cmd.spawn().map_err(|e| format!("Failed to start FFmpeg: {}", e))?;

    // Read progress from stderr
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        let mut lines = reader.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            // Parse progress: "frame=   90 fps= 15 ..."
            if line.contains("frame=") {
                if let Some(frame_str) = line.split("frame=").nth(1) {
                    if let Some(num_str) = frame_str.split_whitespace().next() {
                        if let Ok(frame) = num_str.trim().parse::<usize>() {
                            let percent = (frame as f32 / config.total_frames as f32 * 100.0).min(100.0);
                            let _ = app_handle.emit("conversion-progress", (frame, percent));
                        }
                    }
                }
            }
        }
    }

    let status = child
        .wait()
        .await
        .map_err(|e| format!("FFmpeg execution error: {}", e))?;

    if !status.success() {
        // Delete partial GIF on failure
        let _ = std::fs::remove_file(&config.output_path);
        return Err("FFmpeg conversion failed".to_string());
    }

    Ok(())
}
