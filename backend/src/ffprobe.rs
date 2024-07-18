use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FFProbeInfo {
    streams: Vec<StreamInfo>,
}

#[derive(Debug, Deserialize)]
struct StreamInfo {
    codec_type: String,
    codec_name: String,
    index: u32,
    tags: Option<StreamTags>,
}

#[derive(Default, Debug, Deserialize)]
struct StreamTags {
    language: Option<String>,
    title: Option<String>,
}

pub fn get_track_data(file_path: &str) -> Result<(Vec<String>, Vec<String>)> {
    let pb = PathBuf::from_str(file_path)?;
    Ok((list_audio_tracks(&pb)?, list_subtitle_tracks(&pb)?))
}

pub fn list_audio_tracks(file_path: &PathBuf) -> Result<Vec<String>> {
    // TODO: return a dict and avoid string split to extract value

    let output = Command::new("ffprobe")
        .args(["-print_format", "json"])
        .args(["-show_streams"])
        .args(["-show_format"])
        .args(["-select_streams", "a"])
        .args([file_path])
        .output()?;

    let data: FFProbeInfo = serde_json::from_slice(&output.stdout)?;

    let mut audio_tracks = Vec::new();
    for t in data.streams {
        if t.codec_type != "audio" {
            continue;
        }
        let index = t.index;
        let tags = t.tags.unwrap_or_default();
        let lang = tags.language.unwrap_or_default();
        let title = tags.title.unwrap_or_default();
        let codec = t.codec_name;

        audio_tracks.push(format!("{index}:'{title}' ({lang} - {codec})"));
    }

    Ok(audio_tracks)
}

pub fn list_subtitle_tracks(file_path: &PathBuf) -> Result<Vec<String>> {
    // TODO: return a dict and avoid string split to extract value

    let output = Command::new("ffprobe")
        .args(["-print_format", "json"])
        .args(["-show_streams"])
        .args(["-show_format"])
        .args(["-select_streams", "s"])
        .args([file_path])
        .output()?;

    let data: FFProbeInfo = serde_json::from_slice(&output.stdout)?;

    let mut subtitle_tracks = vec![String::new()];
    for (i, t) in data.streams.into_iter().enumerate() {
        if t.codec_type != "subtitle" {
            continue;
        }
        let tags = t.tags.unwrap_or_default();
        let lang = tags.language.unwrap_or_default();
        let title = tags.title.unwrap_or_default();
        let codec = t.codec_name;

        subtitle_tracks.push(format!("{i}:'{title}' ({lang} - {codec})"));
    }

    Ok(subtitle_tracks)
}
