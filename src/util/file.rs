use std::{error::Error, fs, path::{Path, PathBuf}};

use dirs::home_dir;

/// Returns the absolute path by appending the given relative path to the user's home directory.
/// 
/// ## Arguments
/// 
/// * `path` - A relative path to be appended to the home directory.
/// 
/// ## Returns
/// 
/// * `Result<path::PathBuf, Box<dyn std::error::Error>>` - The absolute path or an error if the home directory cannot be determined.
pub fn get_absolute_path(path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let mut absolute_path = home_dir().ok_or("Failed to determine the home director")?;
    absolute_path.push(path);
    Ok(absolute_path) 
}

/// Ensures that the directory for the given path exists, creating it if necessary.
/// 
/// ## Arguments
/// 
/// * `path` - The path for which to ensure the existence of the parent directory.
/// 
/// ## Returns
/// 
/// * `Result<(), Box<dyn std::error::Error>>` - Ok(()) if the directory exists or was created, or an error if directory creation fails.
pub fn ensure_directory_exists(path: &Path) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
} 