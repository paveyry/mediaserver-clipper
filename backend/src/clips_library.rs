use std::borrow::ToOwned;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::SystemTime;

use common::ClipInfo;

use anyhow::Result;

pub fn audio_clips_in_directory(
    p: &PathBuf,
    public_url_prefix: &str,
    pending: &HashSet<String>,
) -> Result<Vec<ClipInfo>> {
    clips_in_directory_with_ext(p, public_url_prefix, pending, "mp3")
}

pub fn video_clips_in_directory(
    p: &PathBuf,
    public_url_prefix: &str,
    pending: &HashSet<String>,
) -> Result<Vec<ClipInfo>> {
    clips_in_directory_with_ext(p, public_url_prefix, pending, "mp4")
}

fn clips_in_directory_with_ext(
    p: &PathBuf,
    public_url_prefix: &str,
    pending: &HashSet<String>,
    ext: &str,
) -> Result<Vec<ClipInfo>> {
    Ok(files_in_directory_with_ext(p, ext, pending)?
        .iter()
        .map(|(fname, time)| ClipInfo::new(fname.to_owned(), time.to_owned(), public_url_prefix))
        .collect())
}

fn files_in_directory_with_ext(
    p: &PathBuf,
    ext: &str,
    pending: &HashSet<String>,
) -> Result<Vec<(String, SystemTime)>> {
    Ok(fs::read_dir(p)?
        .filter_map(Result::ok)
        .map(|de| {
            (
                de.file_name().to_str().map(ToOwned::to_owned),
                de.metadata()
                    .ok()
                    .and_then(|m| m.modified().ok())
                    .unwrap_or_else(SystemTime::now),
            )
        })
        .filter(|de| de.0.is_some())
        .map(|(fname, t)| (fname.unwrap(), t))
        .filter(|e| e.0.ends_with(ext))
        .filter(|s| !pending.contains(&s.0))
        .collect())
}

pub fn delete_file(dir: &Path, file_name: &str) -> Result<()> {
    let f = dir.join(PathBuf::from_str(file_name)?);
    Ok(fs::remove_file(f)?)
}
