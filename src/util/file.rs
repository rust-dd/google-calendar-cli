use std::{fs, path::{Path, PathBuf}};

use dirs::home_dir;

pub fn get_absolute_path(path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut absolute_path = home_dir().ok_or("Failed to get home directory")?;
    absolute_path.push(path);
    Ok(absolute_path) 
}

pub fn ensure_directory_exists(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
} 