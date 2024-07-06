mod clipper;
mod config;
mod ffprobe;
mod files;
mod models;

use crate::clipper::{validate_start_end, Job};
use crate::config::AppConfig;
use crate::ffprobe::get_track_data;

use rocket::form::{Contextual, Form};
use rocket::fs::{relative, FileServer, Options};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket::{get, launch, post, routes, uri};
use rocket_dyn_templates::{context, Template};

struct App {
    config: AppConfig,
    clipper: clipper::Worker,
}

impl App {
    fn new() -> Self {
        let config = AppConfig::read_from_env();
        let max_queue_size = config.max_queue_size;
        Self {
            config,
            clipper: clipper::Worker::new(max_queue_size),
        }
    }
}

#[launch]
fn app() -> _ {
    let app = App::new();
    let output_dir = app.config.out_path.clone();
    rocket::build()
        .attach(Template::fairing())
        .manage(App::new())
        .mount(
            config::OUTPUT_ROUTE,
            FileServer::new(output_dir, Options::Missing | Options::NormalizeDirs),
        )
        .mount(
            "/public",
            FileServer::new(
                relative!("/public"),
                Options::Missing | Options::NormalizeDirs,
            ),
        )
        .mount(
            "/",
            routes![
                root,
                configure_clip,
                delete_clip,
                create_clip,
                select_source
            ],
        )
}

#[get("/")]
async fn root(app: &State<App>) -> Template {
    let pending = app.clipper.jobs_in_progress();
    match files::video_pairs_in_directory(
        &app.config.out_path,
        &app.config.public_link_prefix,
        &pending,
    ) {
        Ok(clips) => Template::render(
            "root",
            context! { app_name: &app.config.app_name, clips: clips, pending_jobs: pending},
        ),
        Err(e) => render_error(vec![format!(
            "Failed to list clips from output directory: {}",
            e
        )]),
    }
}

#[post("/", data = "<form>")]
async fn select_source(
    form: Form<Contextual<'_, models::NewClipRequest>>,
) -> Result<Flash<Redirect>, Template> {
    if let Some(ref ncr) = form.value {
        let message = Flash::success(Redirect::to(uri!(configure_clip)), ncr.file_path.clone());
        return Ok(message);
    }
    Err(render_error(vec![
        "Path to source file should not be empty".to_string(),
    ]))
}

#[get("/configure_clip")]
async fn configure_clip(app: &State<App>, flash: Option<FlashMessage<'_>>) -> Template {
    let source_file = flash.map_or_else(String::default, |msg| msg.message().to_string());
    if source_file.is_empty() {
        return render_error(vec!["Path to source file should not be empty".to_string()]);
    }
    match get_track_data(&source_file) {
        Ok((at, st)) => Template::render(
            "configure",
            context! {app_name: &app.config.app_name, source_file, audio_tracks: at, subtitle_tracks: st},
        ),
        Err(e) => render_error(vec![format!("failed to get track data from file: {}", e)]),
    }
}

#[post("/configure_clip", data = "<form>")]
async fn create_clip(
    app: &State<App>,
    form: Form<Contextual<'_, models::ConfigureClipRequest>>,
) -> Template {
    if let Some(ref ccr) = form.value {
        match setup_job(app, ccr) {
            Ok(()) => render_message(format!(
                "clip {} was successfully added to processing queue",
                ccr.clip_name
            )),
            Err(e) => render_error(vec![e.to_string()]),
        }
    } else {
        render_error(
            form.context
                .errors()
                .map(|error| {
                    let name = error
                        .name
                        .as_ref()
                        .map_or_else(String::new, ToString::to_string);
                    let description = error;
                    format!("'{name}' {description}")
                })
                .collect::<Vec<_>>(),
        )
    }
}

#[get("/delete?<clip_name>")]
async fn delete_clip(clip_name: String, app: &State<App>) -> Template {
    match files::delete_file(&app.config.out_path, &clip_name) {
        Ok(()) => render_message(format!("Clip {clip_name} was successfully removed")),
        Err(e) => render_error(vec![format!("failed to remove file: {e}")]),
    }
}

fn setup_job(app: &State<App>, ccr: &models::ConfigureClipRequest) -> anyhow::Result<()> {
    let (start_time, end_time) = validate_start_end(
        app.config.max_clip_duration,
        &ccr.start_hour,
        &ccr.start_min,
        &ccr.start_sec,
        &ccr.end_hour,
        &ccr.end_min,
        &ccr.end_sec,
    )?;

    let out_file_path = format!(
        "{}/{}.mp4",
        app.config
            .out_path
            .to_str()
            .ok_or_else(|| anyhow::Error::msg(
                "internal error: output path could not be read as string"
            ))?,
        ccr.clip_name
    );

    app.clipper.add_job(Job::new(
        ccr.source_file.to_string(),
        out_file_path,
        ccr.clip_name.to_string(),
        ccr.audio_track.to_string(),
        ccr.subtitle_track.to_string(),
        start_time,
        end_time,
    ))?;
    Ok(())
}

fn render_error(errors: Vec<String>) -> Template {
    Template::render("message", context! {errors})
}

fn render_message(message: String) -> Template {
    Template::render("message", context! {message})
}
