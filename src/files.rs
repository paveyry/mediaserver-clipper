use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use crate::config::OUTPUT_ROUTE;

use anyhow;
use serde::Serialize;

#[derive(Default, Debug, Serialize)]
pub struct VideoInfo {
    pub(crate) clip_name: String,
    pub(crate) url: String,
    pub(crate) public_url: String,
}

impl VideoInfo {
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
) -> anyhow::Result<Vec<(VideoInfo, VideoInfo)>> {
    Ok(files_in_directory_with_ext(p, ".mp4")?
        .chunks(2)
        .map(|x| {
            if x.len() >= 2 {
                (
                    VideoInfo::new(x[0].to_string(), public_url_prefix),
                    VideoInfo::new(x[1].to_string(), public_url_prefix),
                )
            } else {
                (
                    VideoInfo::new(x[0].to_string(), public_url_prefix),
                    VideoInfo::default(),
                )
            }
        })
        .collect())
}

fn files_in_directory_with_ext(p: &PathBuf, ext: &str) -> anyhow::Result<Vec<String>> {
    Ok(fs::read_dir(p)?
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|de| de.file_name().to_str().map(|s| s.to_owned()))
        .filter(|e| e.ends_with(ext))
        .map(|s| s[..s.len() - 4].to_owned())
        .collect())
}

pub fn delete_file(dir: &PathBuf, clip_name: &str) -> anyhow::Result<()> {
    let f = dir.join(PathBuf::from_str(format!("{clip_name}.mp4").as_str())?);
    Ok(fs::remove_file(f)?)
}
