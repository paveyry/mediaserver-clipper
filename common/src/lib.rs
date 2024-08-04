use std::time::SystemTime;

use serde::{Deserialize, Serialize};

pub mod constants {
    pub const OUTPUT_ROUTE: &str = "/output";
    pub const DEFAULT_APP_NAME: &str = "Media Server Clipper";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app_name: String,
    pub search_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_name: crate::constants::DEFAULT_APP_NAME.to_string(),
            search_enabled: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ClipInfo {
    pub clip_name: String,
    pub url: String,
    pub public_url: String,
    pub file_name: String,
    pub time: SystemTime,
}

impl ClipInfo {
    pub fn new(file_name: String, time: SystemTime, public_url_prefix: &str) -> Self {
        Self {
            url: format!("{}/{}", constants::OUTPUT_ROUTE, file_name),
            public_url: format!("{public_url_prefix}/{file_name}"),
            clip_name: file_name[0..file_name.len() - 4].to_owned(),
            file_name,
            time,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClipsLibrary {
    pub video: Vec<ClipInfo>,
    pub audio: Vec<ClipInfo>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchRequest {
    pub search_string: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FFProbeRequest {
    pub file_path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct FFProbeResult {
    pub audio_tracks: Vec<String>,
    pub sub_tracks: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConfigureClipRequest {
    pub source_file: String,
    pub clip_name: String,
    pub audio_track: String,
    pub subtitle_track: String,
    pub start_sec: String,
    pub start_min: String,
    pub start_hour: String,
    pub end_sec: String,
    pub end_min: String,
    pub end_hour: String,
    pub audio_only: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteClipRequest {
    pub clip_name: String,
}
