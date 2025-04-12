use std::{fs, path::{self, Path, PathBuf}};
use crate::Result;

// function that scans the directory and returns the entry that has to be a directory, with the specific name if it exists
// It will return Result<Option<String>> where the string is the path to the file
// If such directory does not exists, it will return Ok(None)
pub fn find_subdir_with_name(dir: &path::Path, name: &str) -> Result<Option<String>> {
    let read_dir = fs::read_dir(dir)?;
    for entry in read_dir {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().unwrap() == name {
                return Ok(Some(path.to_str().unwrap().to_string()));
            }
        }
    }
    Ok(None)
}

// function returns the latest file in the directory
// It is assumed that the files are named and the latest file is the one that is first in the sorted list
// The files have a timestamp in the name so we can sort them alplphabetically
pub fn get_latest_filename(dir_name: &str) -> Result<Option<PathBuf>> {
    let mut entries: Vec<_> = fs::read_dir(dir_name)?
        .filter_map(|entry| entry.ok())
        .filter(|entry|
            entry
                .file_type()
                .map(|t| t.is_file())
                .unwrap_or(false)
        )
        .map(|entry| entry.file_name().to_string_lossy()
        .into_owned())
        .collect();

    entries.sort_by(|a,b| b.cmp(a));

    if let Some(latest_file) = entries.first() {
        let file_path = Path::new(dir_name).join(latest_file);
        Ok(Some(file_path))
    }
    else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_find_subdir_with_name() {
        let dir = tempdir().unwrap();
        let subdir_name = "subdir";
        let subdir_path = dir.path().join(subdir_name);
        fs::create_dir_all(&subdir_path).unwrap();

        // Existing directory
        let result = find_subdir_with_name(dir.path(), subdir_name).unwrap();
        assert_eq!(result.unwrap(), subdir_path.to_str().unwrap());

        // Non-existing directory
        let non_existent_subdir_name = "non_existent_subdir";
        let result = find_subdir_with_name(dir.path(), non_existent_subdir_name).unwrap();
        assert_eq!(result, None); 
    }

    #[test]
    fn test_get_latest_filename () {
        let dir = tempdir().unwrap();
        let file1 = dir.path().join("file1");
        let file2 = dir.path().join("file2");
        fs::File::create(&file1).unwrap();
        fs::File::create(&file2).unwrap();

        let result = get_latest_filename(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(result.unwrap(), file2);
    }
}