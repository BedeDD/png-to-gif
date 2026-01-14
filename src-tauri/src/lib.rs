mod commands;
mod ffmpeg;
mod sequence;

use commands::{analyze_png_sequence, check_ffmpeg_installed, start_conversion};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            check_ffmpeg_installed,
            analyze_png_sequence,
            start_conversion
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
