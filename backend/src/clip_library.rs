use std::borrow::ToOwned;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

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
    ext: &str
) -> Result<Vec<ClipInfo>> {
    Ok(files_in_directory_with_ext(p, ext, pending)?
        .iter()
        .map(|x| ClipInfo::new(x.to_owned(), public_url_prefix))
        .collect())
}
// pub fn video_pairs_in_directory(
//     p: &PathBuf,
//     public_url_prefix: &str,
//     pending: &HashSet<String>,
// ) -> Result<Vec<(ClipInfo, ClipInfo)>> {
//     Ok(files_in_directory_with_ext(p, "mp4", pending)?
//         .chunks(2)
//         .map(|x| {
//             if x.len() >= 2 {
//                 (
//                     ClipInfo::new(x[0].to_string(), public_url_prefix),
//                     ClipInfo::new(x[1].to_string(), public_url_prefix),
//                 )
//             } else {
//                 (
//                     ClipInfo::new(x[0].to_string(), public_url_prefix),
//                     ClipInfo::default(),
//                 )
//             }
//         })
//         .collect())
// }

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
