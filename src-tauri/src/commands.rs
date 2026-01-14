use crate::ffmpeg::{check_ffmpeg, convert_to_gif, ConversionConfig, FfmpegInfo};
use crate::sequence::{analyze_sequence, SequenceInfo};
use serde::Deserialize;

#[tauri::command]
pub fn check_ffmpeg_installed() -> FfmpegInfo {
    check_ffmpeg()
}

#[tauri::command]
pub fn analyze_png_sequence(paths: Vec<String>) -> SequenceInfo {
    analyze_sequence(paths)
}

#[derive(Deserialize)]
pub struct ConversionRequest {
    pub sequence_info: SequenceInfo,
    pub framerate: u32,
    pub width: u32,
    pub loop_forever: bool,
    pub output_path: String,
}

#[tauri::command]
pub async fn start_conversion(
    request: ConversionRequest,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    // Validate FFmpeg is available
    let ffmpeg_info = check_ffmpeg();
    if !ffmpeg_info.installed {
        return Err("FFmpeg is not installed or not found in PATH".to_string());
    }

    // Validate parameters
    if request.framerate == 0 || request.framerate > 120 {
        return Err("Framerate must be between 1 and 120".to_string());
    }

    if request.width == 0 || request.width > 10000 {
        return Err("Width must be between 1 and 10000".to_string());
    }

    // Build conversion config
    let config = ConversionConfig {
        input_pattern: request.sequence_info.pattern,
        start_number: request.sequence_info.start_number,
        framerate: request.framerate,
        width: request.width,
        loop_forever: request.loop_forever,
        output_path: request.output_path.clone(),
        directory: request.sequence_info.directory,
        total_frames: request.sequence_info.frame_count,
    };

    // Start conversion
    match convert_to_gif(config.clone(), app_handle).await {
        Ok(_) => Ok(()),
        Err(e) => {
            // Clean up partial file on error
            let _ = std::fs::remove_file(&request.output_path);
            Err(e)
        }
    }
}
