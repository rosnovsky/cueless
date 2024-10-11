mod audio;
mod files;
mod strings;
mod ui;

use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let artist_dir = ui::select_artist_directory()?;
    let artist_name = artist_dir.file_name().unwrap().to_string_lossy();
    let slugified_artist_name = strings::slugify(&artist_name);

    let output_dir = Path::new("/mnt/plex/Library/Music").join(&slugified_artist_name);
    std::fs::create_dir_all(&output_dir)?;

    println!("Processing {}...", artist_name);

    if let Err(e) = audio::process_cue_files(&artist_dir, &output_dir) {
        eprintln!("Error processing files: {}", e);
        return Err(e);
    }

    println!(
        "All .cue files for {} have been processed successfully.",
        artist_name
    );
    files::collect_stats(&output_dir, &artist_name);

    if ui::confirm_delete_source(&artist_dir)? {
        std::fs::remove_dir_all(artist_dir)?;
        println!("Source folder deleted successfully.");
    } else {
        println!("Source folder not deleted.");
    }

    Ok(())
}
