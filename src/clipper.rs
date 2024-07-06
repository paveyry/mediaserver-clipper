use std::collections::HashSet;
use std::fmt;
use std::process::Command;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use anyhow;

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

    fn from_strings(hours: &str, minutes: &str, seconds: &str) -> anyhow::Result<Self> {
        Ok(Self(
            str_to_u8(hours)?,
            str_to_u8(minutes)?,
            str_to_u8(seconds)?,
        ))
    }

    fn seconds(&self) -> u32 {
        self.0 as u32 * 3600 + self.1 as u32 * 60 + self.2 as u32
    }
}

fn str_to_u8(s: &str) -> anyhow::Result<u8> {
    if s == "" {
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
    audio_track: String,
    subtitle_track: String,
    start_time: VideoTime,
    end_time: VideoTime,
}

impl Job {
    fn new(
        source_file_path: String,
        out_file_path: String,
        clip_name: String,
        audio_track: String,
        subtitle_track: String,
        start_time: VideoTime,
        end_time: VideoTime,
    ) -> Self {
        Job {
            source_file_path,
            out_file_path,
            clip_name,
            audio_track,
            subtitle_track,
            start_time,
            end_time,
        }
    }
}

pub struct Worker {
    pending_jobs: Arc<Mutex<HashSet<String>>>,
    tx: mpsc::Sender<Job>,
}

impl Worker {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let new_worker = Self {
            pending_jobs: Arc::new(Mutex::new(HashSet::new())),
            tx,
        };
        let pending_jobs_arc = Arc::clone(&new_worker.pending_jobs);
        thread::spawn(move || {
            work(rx, pending_jobs_arc);
        });
        new_worker
    }

    pub fn jobs_in_progress(&self) -> HashSet<String> {
        if let Ok(r) = self.pending_jobs.lock() {
            r.clone()
        } else {
            HashSet::default()
        }
    }

    pub fn add_job(&self, job: Job) -> anyhow::Result<()> {
        let mut job = job;
        // avoid duplicate job ids
        let mut job_id = job.clip_name.clone();
        let mut copy_idx = 0;
        while self.pending_jobs.lock().unwrap().contains(&job_id) {
            copy_idx += 1;
            job_id = format!("{}_copy{}", job.clip_name, copy_idx);
        }
        job.clip_name = job_id;

        self.pending_jobs
            .lock()
            .unwrap()
            .insert(job.clip_name.clone());
        self.tx.send(job)?;
        Ok(())
    }
}

fn work(rx: mpsc::Receiver<Job>, pending_jobs: Arc<Mutex<HashSet<String>>>) {
    println!("worker has started...");
    while let Ok(job) = rx.recv() {
        if let Err(e) = run_job(job, Arc::clone(&pending_jobs)) {
            println!("{}", e);
        }
    }
}

fn run_job(job: Job, pending_jobs: Arc<Mutex<HashSet<String>>>) -> anyhow::Result<()> {
    // let mut args = vec![
    //     "-i".to_string(),
    //     job.file_path.clone(),
    //     "-ss".to_string(),
    //     job.start_time.to_string(),
    //     "-to".to_string(),
    //     job.end_time.to_string(),
    //     "-c:v".to_string(),
    //     "libx264".to_string(),
    //     "-c:a".to_string(),
    //     "acc".to_string(),
    //     "-f".to_string(),
    //     "mp4".to_string(),
    //     "-metadata:g:0".to_string(),
    //     format!("title={}", job.clip_name),
    // ];
    // if !job.subtitle_track.is_empty() {

    // }

    let mut cmd = Command::new("ffmpeg");

    cmd.args(["-i", &job.source_file_path])
        .args(["-ss", &job.start_time.to_string()])
        .args(["-to", &job.end_time.to_string()])
        .args(["-preset", "fast"])
        .args(["-map_metadata", "-1"])
        .args(["-map", "0:0"])
        .args(["-c:v", "libx264"])
        .args(["-c:a", "aac"])
        .args(["-f", "mp4"])
        .args([
            "-metadata:g:0".to_string(),
            format!("title={}", &job.clip_name),
        ])
        .args(["-crf", "22"])
        .args(["-pix_fmt", "yuv420p"]);

    if let Some((a_idx, _)) = job.audio_track.split_once(":") {
        cmd.args(["-map".to_string(), format!("0:{}", a_idx)]);
    } else {
        Err(anyhow::Error::msg("audio_track is missing or invalid"))?;
    }

    if !job.subtitle_track.is_empty() {
        if let Some((st_idx, _)) = job.subtitle_track.split_once(":") {
            cmd.args([
                "-vf".to_string(),
                format!(
                    "subtitles='{}':force_style='FontName=DejaVu Sans':si={}",
                    job.source_file_path, st_idx
                ),
            ]);
        }
    }

    cmd.arg(&job.out_file_path);

    cmd.output()?;

    pending_jobs.lock().unwrap().remove(&job.clip_name);
    Ok(())
}
