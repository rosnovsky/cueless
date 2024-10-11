use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

pub fn collect_stats(output_dir: &Path, artist_name: &str) {
    let mut album_count = 0;
    let mut song_count = 0;
    let mut total_size = 0;
    let mut total_duration = 0.0;

    for entry in WalkDir::new(output_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() && entry.depth() == 1 {
            album_count += 1;
        } else if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "flac" || ext == "mp3" {
                    song_count += 1;
                    if let Ok(metadata) = entry.metadata() {
                        total_size += metadata.len();
                    }
                    if let Ok(duration) = get_audio_duration(entry.path()) {
                        total_duration += duration;
                    }
                }
            }
        }
    }

    println!("Statistics for {}:", artist_name);
    println!("  Albums processed: {}", album_count);
    println!("  Songs processed: {}", song_count);
    println!("  Total file size: {}", format_size(total_size));
    println!("  Total runtime: {}", format_time(total_duration as u64));
}

fn get_audio_duration(file_path: &Path) -> std::io::Result<f64> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v",
            "quiet",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            file_path.to_str().unwrap(),
        ])
        .output()?;

    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<f64>()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

fn format_time(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}
