use regex::Regex;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct SequenceInfo {
    pub valid: bool,
    pub pattern: String,
    pub frame_count: usize,
    pub start_number: usize,
    pub prefix: String,
    pub directory: String,
    pub error: Option<String>,
}

pub fn analyze_sequence(paths: Vec<String>) -> SequenceInfo {
    match analyze_sequence_internal(paths) {
        Ok(info) => info,
        Err(error) => SequenceInfo {
            valid: false,
            pattern: String::new(),
            frame_count: 0,
            start_number: 0,
            prefix: String::new(),
            directory: String::new(),
            error: Some(error),
        },
    }
}

fn analyze_sequence_internal(paths: Vec<String>) -> Result<SequenceInfo, String> {
    // Convert to PathBuf and filter .png files
    let png_files: Vec<PathBuf> = paths
        .iter()
        .map(PathBuf::from)
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("png"))
        .collect();

    if png_files.is_empty() {
        return Err("No PNG files found".to_string());
    }

    if png_files.len() < 2 {
        return Err("Need at least 2 PNG files for a sequence".to_string());
    }

    // Get directory from first file
    let directory = png_files[0]
        .parent()
        .and_then(|p| p.to_str())
        .ok_or("Could not determine directory")?
        .to_string();

    // Parse first file to detect pattern
    let first_name = png_files[0]
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid filename")?;

    let re = Regex::new(r"^(.*?)(\d+)$").unwrap();
    let captures = re
        .captures(first_name)
        .ok_or("Could not detect numbering pattern. Files must end with numbers (e.g., frame_001.png)")?;

    let prefix = captures.get(1).unwrap().as_str();
    let first_num_str = captures.get(2).unwrap().as_str();
    let padding = first_num_str.len();

    // Validate all files match pattern and collect numbers
    let mut numbers = Vec::new();

    for path in &png_files {
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("Invalid filename")?;

        if !stem.starts_with(prefix) {
            return Err(format!(
                "Files have different prefixes. Expected '{}' but found '{}'",
                prefix, stem
            ));
        }

        let num_part = &stem[prefix.len()..];

        // Check consistent padding
        if num_part.len() != padding {
            return Err(format!(
                "Inconsistent padding detected. All frame numbers must have the same padding (e.g., all 001, 002, 003 or all 1, 2, 3)"
            ));
        }

        let num: usize = num_part
            .parse()
            .map_err(|_| format!("Invalid frame number in '{}'", stem))?;

        numbers.push(num);
    }

    // Sort and check sequence continuity
    numbers.sort();
    let start = numbers[0];
    let end = numbers[numbers.len() - 1];
    let expected_count = end - start + 1;

    if numbers.len() != expected_count {
        // Find missing frames
        let mut missing = Vec::new();
        for i in start..=end {
            if !numbers.contains(&i) {
                missing.push(i);
            }
        }
        return Err(format!(
            "Sequence has gaps. Missing frames: {}. Found {} frames but expected {} ({}–{})",
            missing.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", "),
            numbers.len(),
            expected_count,
            start,
            end
        ));
    }

    // Build FFmpeg pattern
    let pattern = if padding > 1 {
        format!("{}%0{}d.png", prefix, padding)
    } else {
        format!("{}%d.png", prefix)
    };

    Ok(SequenceInfo {
        valid: true,
        pattern,
        frame_count: numbers.len(),
        start_number: start,
        prefix: prefix.to_string(),
        directory,
        error: None,
    })
}
