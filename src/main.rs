use std::env;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use clipper::Job;
use ffprobe::get_track_data;
use rocket::form::{Contextual, Form};
use rocket::fs::{relative, FileServer, Options};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket::{get, launch, post, routes, uri};
use rocket_dyn_templates::{context, Template};

mod clipper;
mod ffprobe;
mod files;
mod models;

const APP_NAME_VARNAME: &str = "APP_NAME";
const OUTPUT_PATH_VARNAME: &str = "OUTPUT_PATH";
const OUTPUT_LINK_PREFIX_VARNAME: &str = "OUTPUT_LINK_PREFIX";

const DEFAULT_OUTPUT_PATH: &str = "output";
const DEFAULT_VAR_NAME: &str = "Media Server Clipper";

struct AppConfig {
    app_name: String,
    out_path: PathBuf,
    out_link_prefix: String,
}

impl AppConfig {
    fn read_from_env() -> Self {
        let out_path =
            env::var(OUTPUT_PATH_VARNAME).unwrap_or_else(|_| DEFAULT_OUTPUT_PATH.to_string());
        let out_link_prefix =
            env::var(OUTPUT_LINK_PREFIX_VARNAME).unwrap_or_else(|_| format!("/{out_path}"));
        let out_path_buf = PathBuf::from_str(&out_path)
            .expect(format!("{OUTPUT_PATH_VARNAME} should be a valid path").as_str());
        if !fs::metadata(&out_path_buf).is_ok() {
            fs::create_dir_all(&out_path_buf).expect("failed to create missing output directory");
        }
        Self {
            app_name: env::var(APP_NAME_VARNAME).unwrap_or_else(|_| DEFAULT_VAR_NAME.to_string()),
            out_path: out_path_buf,
            out_link_prefix,
        }
    }
}

struct App {
    config: AppConfig,
    clipper: clipper::Worker,
}

impl App {
    fn new() -> Self {
        Self {
            config: AppConfig::read_from_env(),
            clipper: clipper::Worker::new(),
        }
    }
}

#[launch]
fn app() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .manage(App::new())
        .mount(
            "/public",
            FileServer::new(
                relative!("/public"),
                Options::Missing | Options::NormalizeDirs,
            ),
        )
        .mount(
            "/",
            routes![root, configure_clip, create_clip, select_source],
        )
}

#[get("/")]
async fn root(app: &State<App>) -> Template {
    match files::video_pairs_in_directory(&app.config.out_path) {
        Ok(clips) => Template::render(
            "root",
            context! { app_name: &app.config.app_name, clips: clips},
        ),
        Err(e) => render_error(vec![format!(
            "Failed to list clips from output directory: {}",
            e
        )]),
    }
}

#[post("/", data = "<form>")]
async fn select_source(
    app: &State<App>,
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
    let source_file = flash.map_or_else(|| String::default(), |msg| msg.message().to_string());
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
) -> Result<Flash<Redirect>, Template> {
    if let Some(ref ccr) = form.value {
        app.clipper.add_job(Job::new(
            ccr.source_file.to_string(),
            format!("{}/{}", app.config.out_path, ccr.clip_name),
            ccr.clip_name.to_string(),
            audio_track,
            subtitle_track,
            start_time,
            end_time,
        ));

        let message = Flash::success(Redirect::to(uri!(configure_clip)), ccr.source_file.clone());
        return Ok(message);
    }
    Err(render_error(
        form.context
            .errors()
            .map(|error| {
                let name = error
                    .name
                    .as_ref()
                    .map_or_else(|| "".to_string(), |e| e.to_string());
                let description = error;
                format!("'{}' {}", name, description)
            })
            .collect::<Vec<_>>(),
    ))
}

fn render_error(errors: Vec<String>) -> Template {
    Template::render("error", context! {errors})
}
