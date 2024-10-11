use std::io;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

pub fn process_cue_files(artist_dir: &Path, output_dir: &Path) -> io::Result<()> {
    let mut cue_files_found = false;
    let mut error_occurred = false;

    for entry in WalkDir::new(artist_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().and_then(|s| s.to_str()) == Some("cue") {
            cue_files_found = true;
            let album_dir = entry.path().parent().unwrap();
            let album_name = album_dir.file_name().unwrap().to_string_lossy();
            let sanitized_album_name = crate::strings::sanitize_name(&album_name);
            let album_output_dir = output_dir.join(&sanitized_album_name);
            std::fs::create_dir_all(&album_output_dir)?;

            println!("Processing: {}", album_name);
            if let Err(e) = run_ffcuesplitter(entry.path(), &album_output_dir) {
                eprintln!("Error processing: {}", album_name);
                eprintln!("Error: {}", e);
                error_occurred = true;
            } else {
                println!("Processed: {}", album_name);
            }
        }
    }

    if !cue_files_found {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No .cue files found",
        ));
    }

    if error_occurred {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Some errors occurred during processing",
        ));
    }

    Ok(())
}

fn run_ffcuesplitter(cue_file: &Path, output_dir: &Path) -> io::Result<()> {
    let output = Command::new("ffcuesplitter")
        .arg("-i")
        .arg(cue_file)
        .arg("-o")
        .arg(output_dir)
        .arg("-ow always")
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr),
        ));
    }

    Ok(())
}
