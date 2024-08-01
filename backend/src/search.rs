use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::thread;

use anyhow::{Error, Result};
use walkdir::WalkDir;

struct SourceSettings {
    pub dirs: Vec<PathBuf>,
    pub allowed_exts: HashSet<String>,
}

pub struct SearchEngine {
    index_map: Arc<RwLock<HashMap<String, String>>>, // HashMap<filepath_lowercase, filepath>
    refresh_in_progress: Arc<RwLock<bool>>,
    source_settings: Arc<SourceSettings>,
}

impl SearchEngine {
    pub fn new(source_dirs: Vec<PathBuf>, allowed_exts: HashSet<String>) -> Self {
        let new_index = Self {
            index_map: Arc::new(RwLock::new(HashMap::new())),
            refresh_in_progress: Arc::new(RwLock::new(false)),
            source_settings: Arc::new(SourceSettings {
                dirs: source_dirs,
                allowed_exts,
            }),
        };
        new_index
            .refresh_index()
            .expect("initial search engine indexing failed");
        new_index
    }

    pub fn is_refreshing(&self) -> bool {
        *self.refresh_in_progress.read().unwrap()
    }

    pub fn refresh_index(&self) -> Result<()> {
        log::info!("search: indexing files");
        {
            let mut in_progress = self.refresh_in_progress.write().unwrap();
            if *in_progress {
                return Err(Error::msg(
                    "file indexing forbidden: another indexing is already in progress",
                ));
            }
            *in_progress = true;
        }
        let refresh_in_progress = Arc::clone(&self.refresh_in_progress);
        let index_map = Arc::clone(&self.index_map);
        let source_settings = Arc::clone(&self.source_settings);
        thread::spawn(move || {
            let new_index_map = create_index_map(source_settings);
            println!("INDEX: {:?}", &new_index_map);
            *index_map.write().unwrap() = new_index_map;
            *refresh_in_progress.write().unwrap() = false;
        });
        Ok(())
    }

    pub fn search(&self, search_fields: &[&str]) -> Vec<String> {
        if search_fields.is_empty() {
            return Vec::new();
        }

        let n = search_fields.len();

        let mut res = {
            let index_map = self.index_map.read().unwrap();

            if n == 1 {
                index_map
                    .iter()
                    .filter(|(k, _)| k.contains(search_fields[0].to_lowercase().as_str()))
                    .map(|(_, s)| s.to_owned())
                    .collect::<Vec<_>>()
            } else {
                // first pass filter to produce a vector
                let mut vec = index_map
                    .iter()
                    .filter(|(k, _)| k.contains(search_fields[0].to_lowercase().as_str()))
                    .collect::<Vec<_>>();
                // filter further in the vector with the following search fields (except the last to avoid collecting a vector of refs for nothing on the last)
                for fs in &search_fields[1..n - 1] {
                    let lowerfs = fs.to_lowercase();
                    vec = vec
                        .into_iter()
                        .filter(|(k, _)| k.contains(&lowerfs))
                        .collect::<Vec<_>>();
                }
                vec.into_iter()
                    .filter(|(k, _)| k.contains(search_fields[n - 1]))
                    .map(|(_, s)| s.to_owned())
                    .collect::<Vec<_>>()
            }
        }; // release index_map read lock

        res.sort();
        res
    }
}

fn create_index_map(source_settings: Arc<SourceSettings>) -> HashMap<String, String> {
    let mut new_index_map = HashMap::new();

    for source_dir in &source_settings.dirs {
        let walker = WalkDir::new(source_dir).into_iter();
        for entry in walker
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                source_settings.allowed_exts.is_empty()
                    || source_settings.allowed_exts.contains(
                        e.path()
                            .extension()
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_or_default(),
                    )
            })
        {
            let Some(path) = entry.path().to_str() else {
                log::error!("failed to convert dir entry {entry:?}");
                continue;
            };
            let key = path.to_lowercase();
            new_index_map.insert(key, path.to_owned());
        }
    }
    new_index_map
}
