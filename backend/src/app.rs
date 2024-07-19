use std::env;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use crate::clipper;
use crate::search;

const APP_NAME_VARNAME: &str = "APP_NAME"; // TODO: display again
const OUTPUT_PATH_VARNAME: &str = "OUTPUT_PATH";
const PUBLIC_LINK_PREFIX_VARNAME: &str = "PUBLIC_LINK_PREFIX";
const MAX_CLIP_DURATION_VARNAME: &str = "MAX_CLIP_DURATION";
const MAX_QUEUE_SIZE_VARNAME: &str = "MAX_QUEUE_SIZE";
const SEARCH_EXTS_VARNAME: &str = "SEARCH_FILE_EXTS";
pub const SEARCH_DIRS_VARNAME: &str = "SEARCH_DIRS";

const DEFAULT_OUTPUT_PATH: &str = "output";
const DEFAULT_VAR_NAME: &str = "Media Server Clipper";
const DEFAULT_MAX_CLIP_DURATION: u64 = 600; // 10 minutes
const DEFAULT_MAX_QUEUE_SIZE: usize = 4;


pub struct App {
    // components
    pub clipper: clipper::Worker,
    pub search: Option<search::SearchEngine>,

    // config
    pub app_name: String,
    pub out_path: PathBuf,
    pub public_link_prefix: String,
    pub max_clip_duration: u64,
}

impl App {
    pub fn init() -> Self {
        let out_path =
            env::var(OUTPUT_PATH_VARNAME).unwrap_or_else(|_| DEFAULT_OUTPUT_PATH.to_string());

        let out_link_prefix =
            env::var(PUBLIC_LINK_PREFIX_VARNAME).unwrap_or_else(|_| format!("/{out_path}"));

        let out_path_buf = PathBuf::from_str(&out_path)
            .unwrap_or_else(|_| panic!("{OUTPUT_PATH_VARNAME} should be a valid path"));

        let max_clip_duration = env::var(MAX_CLIP_DURATION_VARNAME)
            .map(|d| {
                d.parse().unwrap_or_else(|_| {
                    panic!("{MAX_CLIP_DURATION_VARNAME} should be a valid number of seconds")
                })
            })
            .unwrap_or(DEFAULT_MAX_CLIP_DURATION);

        let max_queue_size = env::var(MAX_QUEUE_SIZE_VARNAME)
            .map(|d| {
                d.parse().unwrap_or_else(|_| {
                    panic!("{MAX_QUEUE_SIZE_VARNAME} should be a valid positive number")
                })
            })
            .unwrap_or(DEFAULT_MAX_QUEUE_SIZE);
        if max_queue_size == 0 {
            panic!("{MAX_QUEUE_SIZE_VARNAME} should not be 0");
        }

        if fs::metadata(&out_path_buf).is_err() {
            fs::create_dir_all(&out_path_buf).expect("failed to create missing output directory");
        }

        let allowed_exts = env::var(SEARCH_EXTS_VARNAME)
            .unwrap_or_default()
            .trim()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(ToOwned::to_owned)
            .collect();
        let source_dirs = env::var(SEARCH_DIRS_VARNAME)
            .unwrap_or_default()
            .trim()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| PathBuf::from_str(s).expect("invalid search directory"))
            .collect::<Vec<_>>();

        Self {
            clipper: clipper::Worker::new(max_queue_size),
            search: if source_dirs.is_empty() {
                None
            } else {
                Some(search::SearchEngine::new(source_dirs, allowed_exts))
            },

            app_name: env::var(APP_NAME_VARNAME).unwrap_or_else(|_| DEFAULT_VAR_NAME.to_string()),
            out_path: out_path_buf,
            public_link_prefix: out_link_prefix,
            max_clip_duration,
        }
    }
}
