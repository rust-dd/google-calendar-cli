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
/// * `Result<PathBuf, Box<dyn std::error::Error>>` - The absolute path or an error if the home directory cannot be determined.
pub fn get_absolute_path(path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let mut absolute_path = home_dir().ok_or("Failed to determine the home directory")?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs};
    use std::path::Path;

    #[test]
    fn test_get_absolute_path() {
        let relative_path = "test_folder/subfolder";
        let absolute_path = get_absolute_path(relative_path)
            .expect("Failed to get absolute path");
        let home = home_dir().expect("Failed to get home directory");

        assert!(absolute_path.starts_with(&home));

        let appended = absolute_path.strip_prefix(&home)
            .expect("Absolute path should have home as prefix");
        assert_eq!(appended, Path::new(relative_path));
    }

    #[test]
    fn test_ensure_directory_exists() {
        let temp_dir = env::temp_dir();
        let unique_dir = temp_dir.join("test_dir_for_ensure_directory_exists");
        let test_file_path = unique_dir.join("test_file.txt");

        let _ = fs::remove_dir_all(&unique_dir);

        ensure_directory_exists(&test_file_path)
            .expect("Failed to ensure directory exists");

        assert!(unique_dir.exists());

        let _ = fs::remove_dir_all(&unique_dir);
    }
}
