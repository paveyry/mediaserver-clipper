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
}

impl ClipInfo {
    fn new(clip_name: String, public_url_prefix: &str) -> Self {
        Self {
            url: format!("{OUTPUT_ROUTE}/{clip_name}.mp4"),
            public_url: format!("{public_url_prefix}/{clip_name}.mp4"),
            clip_name,
        }
    }
}

pub fn video_pairs_in_directory(
    p: &PathBuf,
    public_url_prefix: &str,
    pending: &HashSet<String>,
) -> Result<Vec<(ClipInfo, ClipInfo)>> {
    Ok(files_in_directory_with_ext(p, ".mp4", pending)?
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
        .map(|s| s[..s.len() - 4].to_owned())
        .filter(|s| !pending.contains(s))
        .collect())
}

pub fn delete_file(dir: &Path, clip_name: &str) -> Result<()> {
    let f = dir.join(PathBuf::from_str(format!("{clip_name}.mp4").as_str())?);
    Ok(fs::remove_file(f)?)
}
