use std::env;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

const APP_NAME_VARNAME: &str = "APP_NAME";
const OUTPUT_PATH_VARNAME: &str = "OUTPUT_PATH";
const PUBLIC_LINK_PREFIX_VARNAME: &str = "PUBLIC_LINK_PREFIX";
const MAX_CLIP_DURATION_VARNAME: &str = "MAX_CLIP_DURATION";
const MAX_QUEUE_SIZE_VARNAME: &str = "MAX_CLIP_DURATION";

const DEFAULT_OUTPUT_PATH: &str = "output";
const DEFAULT_VAR_NAME: &str = "Media Server Clipper";
const DEFAULT_MAX_CLIP_DURATION: u64 = 600; // 10 minutes
const DEFAULT_MAX_QUEUE_SIZE: usize = 4;

pub(crate) const OUTPUT_ROUTE: &str = "/output";

pub struct AppConfig {
    pub app_name: String,
    pub out_path: PathBuf,
    pub public_link_prefix: String,
    pub max_clip_duration: u64,
    pub max_queue_size: usize,
}

impl AppConfig {
    pub fn read_from_env() -> Self {
        let out_path =
            env::var(OUTPUT_PATH_VARNAME).unwrap_or_else(|_| DEFAULT_OUTPUT_PATH.to_string());

        let out_link_prefix =
            env::var(PUBLIC_LINK_PREFIX_VARNAME).unwrap_or_else(|_| format!("/{out_path}"));

        let out_path_buf = PathBuf::from_str(&out_path)
            .expect(format!("{OUTPUT_PATH_VARNAME} should be a valid path").as_str());

        let max_clip_duration = env::var(MAX_CLIP_DURATION_VARNAME)
            .map(|d| {
                d.parse().expect(
                    format!("{MAX_CLIP_DURATION_VARNAME} should be a valid number of seconds")
                        .as_str(),
                )
            })
            .unwrap_or(DEFAULT_MAX_CLIP_DURATION);

        let max_queue_size = env::var(MAX_QUEUE_SIZE_VARNAME)
            .map(|d| {
                d.parse().expect(
                    format!("{MAX_QUEUE_SIZE_VARNAME} should be a valid positive number").as_str(),
                )
            })
            .unwrap_or(DEFAULT_MAX_QUEUE_SIZE);
        if max_queue_size == 0 {
            panic!("{MAX_QUEUE_SIZE_VARNAME} should not be 0");
        }

        if !fs::metadata(&out_path_buf).is_ok() {
            fs::create_dir_all(&out_path_buf).expect("failed to create missing output directory");
        }

        Self {
            app_name: env::var(APP_NAME_VARNAME).unwrap_or_else(|_| DEFAULT_VAR_NAME.to_string()),
            out_path: out_path_buf,
            public_link_prefix: out_link_prefix,
            max_clip_duration,
            max_queue_size,
        }
    }
}
