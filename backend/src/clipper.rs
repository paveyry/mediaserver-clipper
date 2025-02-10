use std::collections::HashSet;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;

use crate::app::LOCK_ERROR;

use anyhow::{Context, Error, Result};

#[derive(Debug, Clone)]
pub struct VideoTime(u8, u8, u8);

impl VideoTime {
    fn new(hours: u8, minutes: u8, seconds: u8) -> VideoTime {
        let mut hours = hours;
        let mut minutes = minutes;
        let mut seconds = seconds;
        if seconds >= 60 {
            minutes += seconds / 60;
            seconds %= 60;
        }
        if minutes >= 60 {
            hours += minutes / 60;
            minutes %= 60;
        }
        Self(hours, minutes, seconds)
    }

    fn from_strings(hours: &str, minutes: &str, seconds: &str) -> Result<Self> {
        Ok(Self::new(
            str_to_u8(hours)?,
            str_to_u8(minutes)?,
            str_to_u8(seconds)?,
        ))
    }

    fn seconds(&self) -> u64 {
        self.0 as u64 * 3600 + self.1 as u64 * 60 + self.2 as u64
    }
}

pub fn validate_start_end(
    max_seconds: u64,
    start_hour: &str,
    start_min: &str,
    start_sec: &str,
    end_hour: &str,
    end_min: &str,
    end_sec: &str,
) -> Result<(VideoTime, VideoTime)> {
    let start_time = VideoTime::from_strings(start_hour, start_min, start_sec)?;
    let end_time = VideoTime::from_strings(end_hour, end_min, end_sec)?;

    let duration = end_time.seconds() as i64 - start_time.seconds() as i64;
    if duration <= 0 {
        return Err(Error::msg("start time should be before end time"));
    }
    if duration as u64 > max_seconds {
        return Err(Error::msg(format!(
            "clip duration should not exceed {max_seconds} seconds"
        )));
    }
    Ok((start_time, end_time))
}

fn str_to_u8(s: &str) -> Result<u8> {
    if s.is_empty() {
        return Ok(0);
    }
    Ok(s.parse()?)
}

impl fmt::Display for VideoTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}:{:02}:{:02}", self.0, self.1, self.2))
    }
}

#[derive(Debug, Clone)]
pub struct Job {
    source_file_path: String,
    out_file_path: String,
    clip_name: String,
    file_name: String,
    audio_track: String,
    subtitle_track: String,
    start_time: VideoTime,
    end_time: VideoTime,
    audio_only: bool,
}

impl Job {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source_file_path: String,
        out_file_path: String,
        clip_name: String,
        file_name: String,
        audio_track: String,
        subtitle_track: String,
        start_time: VideoTime,
        end_time: VideoTime,
        audio_only: bool,
    ) -> Self {
        Job {
            source_file_path,
            out_file_path,
            clip_name,
            file_name,
            audio_track,
            subtitle_track,
            start_time,
            end_time,
            audio_only,
        }
    }
}

pub struct Worker {
    pending_jobs: Arc<RwLock<HashSet<String>>>,
    tx: mpsc::Sender<Job>,
    max_queue_size: usize,
    failures: Arc<RwLock<Vec<String>>>,
}

impl Worker {
    pub fn new(max_queue_size: usize) -> Self {
        let (tx, rx) = mpsc::channel();
        let new_worker = Self {
            pending_jobs: Arc::new(RwLock::new(HashSet::new())),
            tx,
            max_queue_size,
            failures: Arc::new(RwLock::new(Vec::new())),
        };
        let pending_jobs_arc = Arc::clone(&new_worker.pending_jobs);
        let failures_arc = Arc::clone(&new_worker.failures);
        thread::spawn(move || {
            work(rx, pending_jobs_arc, failures_arc);
        });
        new_worker
    }

    pub fn clear_failures(&self) {
        *self.failures.write().expect(LOCK_ERROR) = Vec::new();
    }

    pub fn failures(&self) -> Vec<String> {
        self.failures.read().expect(LOCK_ERROR).clone()
    }

    pub fn jobs_in_progress(&self) -> HashSet<String> {
        self.pending_jobs.read().expect(LOCK_ERROR).clone()
    }

    pub fn add_job(&self, job: Job) -> Result<()> {
        {
            let mut pj = self.pending_jobs.write().expect(LOCK_ERROR);

            if pj.len() >= self.max_queue_size {
                return Err(Error::msg(format!(
                    "maximum job queue size has been reached: {}",
                    self.max_queue_size
                )));
            }

            if pj.contains(&job.file_name) {
                return Err(Error::msg(format!(
                    "there is already a pending job called {}",
                    &job.file_name
                )));
            }

            pj.insert(job.file_name.clone());
        }
        self.tx.send(job)?;
        Ok(())
    }
}

fn work(
    rx: mpsc::Receiver<Job>,
    pending_jobs: Arc<RwLock<HashSet<String>>>,
    failures: Arc<RwLock<Vec<String>>>,
) {
    log::info!("worker has started...");
    while let Ok(job) = rx.recv() {
        if let Err(e) = run_job(&job) {
            let err_msg = format!("clip '{}': {}", job.clip_name, e);
            failures
                .write()
                .expect("fatal error; lock holder has panicked")
                .push(err_msg);
            log::error!("{e}");
        }
        pending_jobs
            .write()
            .expect("fatal error; lock holder has panicked")
            .remove(&job.file_name);
    }
}

fn run_job(job: &Job) -> Result<()> {
    log::info!("starting file encoding: {}", &job.clip_name);

    if !Path::exists(
        &PathBuf::from_str(&job.source_file_path)
            .context(format!("invalid file path {}", &job.source_file_path))?,
    ) {
        return Err(Error::msg(
            format! {"file {} does not exist", &job.source_file_path},
        ));
    }

    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-y")
        .args(["-i", &job.source_file_path])
        .args(["-ss", &job.start_time.to_string()])
        .args(["-to", &job.end_time.to_string()])
        .args(["-map_metadata", "-1"])
        .args([
            "-metadata:g:0".to_string(),
            format!("title={}", &job.clip_name),
        ]);

    if job.audio_only {
        cmd.args(["-c:a", "mp3"]).args(["-f", "mp3"]);
    } else {
        cmd.args(["-c:v", "libx264"])
            .args(["-c:a", "aac"])
            .args(["-map", "0:0"])
            .args(["-f", "mp4"])
            .args(["-crf", "22"])
            .args(["-pix_fmt", "yuv420p"]);
    }

    if let Some((a_idx, _)) = job.audio_track.split_once(':') {
        cmd.args(["-map".to_string(), format!("0:{}", a_idx)]);
    } else {
        Err(Error::msg("audio_track is missing or invalid"))?;
    }

    if !job.subtitle_track.is_empty() && !job.audio_only {
        if let Some((st_idx, _)) = job.subtitle_track.split_once(':') {
            cmd.args([
                "-vf".to_string(),
                format!(
                    "subtitles='{}':force_style='FontName=DejaVu Sans':si={}",
                    job.source_file_path, st_idx
                ),
            ]);
        }
    }

    cmd.arg(&job.out_file_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let output = cmd.output()?;
    if !output.status.success() {
        return Err(Error::msg(format!(
            "job {} failed: ffmpeg returned status {}",
            &job.clip_name, output.status
        )));
    }

    log::info!("file transcoding succeded: {}", &job.clip_name);
    Ok(())
}
