use dialoguer::{Confirm, Select};
use std::io;
use std::path::{Path, PathBuf};

pub fn select_artist_directory() -> io::Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let dirs: Vec<_> = std::fs::read_dir(&current_dir)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                if e.file_type().ok()?.is_dir() {
                    Some(e.path())
                } else {
                    None
                }
            })
        })
        .collect();

    if dirs.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No directories found.",
        ));
    }

    let dir_strings: Vec<String> = dirs
        .iter()
        .map(|path| path.to_string_lossy().into_owned())
        .collect();

    let selection = Select::new()
        .with_prompt("Select an artist folder to process")
        .items(&dir_strings)
        .interact()?;

    Ok(dirs[selection].clone())
}

pub fn confirm_delete_source(artist_dir: &Path) -> io::Result<bool> {
    if Confirm::new()
        .with_prompt("Is it okay to delete the source folder?")
        .default(false)
        .interact()?
    {
        Confirm::new()
            .with_prompt(format!("Are you sure you want to delete {:?}?", artist_dir))
            .default(false)
            .interact()
    } else {
        Ok(false)
    }
}
