use std::borrow::ToOwned;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::app::OUTPUT_ROUTE;

use anyhow::Result;
use serde::Serialize;

#[derive(Default, Debug, Serialize)]
pub struct ClipInfo {
    pub(crate) clip_name: String,
    pub(crate) url: String,
    pub(crate) public_url: String,
    pub(crate) file_name: String,
}

impl ClipInfo {
    fn new(file_name: String, public_url_prefix: &str) -> Self {
        Self {
            url: format!("{OUTPUT_ROUTE}/{file_name}"),
            public_url: format!("{public_url_prefix}/{file_name}"),
            clip_name: file_name[0..file_name.len() - 4].to_owned(),
            file_name,
        }
    }
}

pub fn audio_clips_in_directory(
    p: &PathBuf,
    public_url_prefix: &str,
    pending: &HashSet<String>,
) -> Result<Vec<ClipInfo>> {
    Ok(files_in_directory_with_ext(p, "mp3", pending)?
        .iter()
        .map(|x| ClipInfo::new(x.to_owned(), public_url_prefix))
        .collect())
}

pub fn video_pairs_in_directory(
    p: &PathBuf,
    public_url_prefix: &str,
    pending: &HashSet<String>,
) -> Result<Vec<(ClipInfo, ClipInfo)>> {
    Ok(files_in_directory_with_ext(p, "mp4", pending)?
        .chunks(2)
        .map(|x| {
            if x.len() >= 2 {
                (
                    ClipInfo::new(x[0].to_string(), public_url_prefix),
                    ClipInfo::new(x[1].to_string(), public_url_prefix),
                )
            } else {
                (
                    ClipInfo::new(x[0].to_string(), public_url_prefix),
                    ClipInfo::default(),
                )
            }
        })
        .collect())
}

fn files_in_directory_with_ext(
    p: &PathBuf,
    ext: &str,
    pending: &HashSet<String>,
) -> Result<Vec<String>> {
    Ok(fs::read_dir(p)?
        .filter_map(Result::ok)
        .filter_map(|de| de.file_name().to_str().map(ToOwned::to_owned))
        .filter(|e| e.ends_with(ext))
        .filter(|s| !pending.contains(s))
        .collect())
}

pub fn delete_file(dir: &Path, file_name: &str) -> Result<()> {
    let f = dir.join(PathBuf::from_str(file_name)?);
    Ok(fs::remove_file(f)?)
}
