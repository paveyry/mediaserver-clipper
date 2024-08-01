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

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ClipInfo {
    pub clip_name: String,
    pub url: String,
    pub public_url: String,
    pub file_name: String,
}

impl ClipInfo {
    pub fn new(file_name: String, public_url_prefix: &str) -> Self {
        Self {
            url: format!("{}/{}", constants::OUTPUT_ROUTE, file_name),
            public_url: format!("{public_url_prefix}/{file_name}"),
            clip_name: file_name[0..file_name.len() - 4].to_owned(),
            file_name,
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
    // #[field(validate=len(1..))]
    pub search_string: String,
}