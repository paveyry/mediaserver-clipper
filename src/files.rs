use std::fs::read_dir;
use std::path::PathBuf;

use anyhow;

pub fn videos_in_directory(p: PathBuf) -> anyhow::Result<Vec<String>> {
    files_in_directory_with_ext(p, ".mp4")
}


fn files_in_directory_with_ext(p: PathBuf, ext: &str) -> anyhow::Result<Vec<String>> {
    Ok(read_dir(p)?
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|de| de.file_name().to_str().map(|s| s.to_owned()))
        .filter(|e| e.ends_with(ext))
        .collect())
}

