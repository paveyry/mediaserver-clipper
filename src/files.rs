use std::fs::read_dir;
use std::path::PathBuf;

use anyhow;

pub fn video_pairs_in_directory(p: &PathBuf) -> anyhow::Result<Vec<(String, String)>> {
    Ok(files_in_directory_with_ext(p, ".mp4")?
        .chunks(2)
        .map(|x| {
            if x.len() >= 2 {
                (x[0].to_string(), x[1].to_string())
            } else {
                (x[0].to_string(), String::new())
            }
        })
        .collect())
}

fn files_in_directory_with_ext(p: &PathBuf, ext: &str) -> anyhow::Result<Vec<String>> {
    Ok(read_dir(p)?
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|de| de.file_name().to_str().map(|s| s.to_owned()))
        .filter(|e| e.ends_with(ext))
        .map(|s| s[..s.len() - 4].to_owned())
        .collect())
}
