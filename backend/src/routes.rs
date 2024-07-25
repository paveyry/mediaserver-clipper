use std::path::PathBuf;
use std::result::Result as StdResult;

use crate::app::{App, SEARCH_DIRS_VARNAME};
use crate::clipper::{validate_start_end, Job};
use crate::clips_library;
use crate::ffprobe::get_track_data;
use crate::models;

use anyhow::{Context, Result as AnyResult};
use itertools::Itertools;
use rocket::form::{Contextual, Form};
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket::response::status::{BadRequest, Custom as CustomStatus, Forbidden, NotFound};
use rocket::response::{Flash, Redirect};
use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, post, uri};
use rocket_dyn_templates::{context, Template};

#[get("/<ui_file>")]
pub async fn ui_files(ui_file: PathBuf) -> StdResult<NamedFile, NotFound<String>> {
    let path = PathBuf::from("../ui/dist").join(ui_file);
    NamedFile::open(path)
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/")]
pub async fn root() -> StdResult<NamedFile, NotFound<String>> {
    NamedFile::open("../ui/dist/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/app_config")]
pub async fn app_config(app: &State<App>) -> Json<common::Config> {
    Json(common::Config {
        app_name: app.app_name.to_string(),
        search_enabled: app.search.is_some(),
    })
}

#[get("/clips")]
pub async fn clips(app: &State<App>) -> StdResult<Json<common::ClipsLibrary>, Forbidden<String>> {
    let pending = app.clipper.jobs_in_progress();
    let video =
        clips_library::video_clips_in_directory(&app.out_path, &app.public_link_prefix, &pending)
            .map_err(|e| Forbidden(e.to_string()))?;
    let audio =
        clips_library::audio_clips_in_directory(&app.out_path, &app.public_link_prefix, &pending)
            .map_err(|e| Forbidden(e.to_string()))?;
    Ok(Json(common::ClipsLibrary { video, audio }))
}

// #[get("/")]
// pub async fn root(app: &State<App>) -> Template {
//     let (search_enabled, index_refreshing) = app
//         .search
//         .as_ref()
//         .map_or((false, false), |index| (true, index.is_refreshing()));

//     let pending = app.clipper.jobs_in_progress();
//     let failures = app.clipper.failures();

//     let video_clips =
//         match clips::video_pairs_in_directory(&app.out_path, &app.public_link_prefix, &pending) {
//             Ok(clips) => clips,
//             Err(e) => {
//                 return render_error(vec![format!(
//                     "Failed to list video clips from output directory: {}",
//                     e
//                 )])
//             }
//         };
//     let audio_clips =
//         match clips::audio_clips_in_directory(&app.out_path, &app.public_link_prefix, &pending) {
//             Ok(clips) => clips,
//             Err(e) => {
//                 return render_error(vec![format!(
//                     "Failed to list audio clips from output directory: {}",
//                     e
//                 )])
//             }
//         };

//     Template::render(
//         "root",
//         context! {
//             app_name: &app.app_name,
//             video_clips,
//             audio_clips,
//             failures,
//             pending_jobs: pending,
//             index_refreshing,
//             search_enabled,
//         },
//     )
// }

#[post("/", data = "<form>")]
pub async fn select_source(
    form: Form<Contextual<'_, models::NewClipRequest>>,
) -> Result<Flash<Redirect>, Template> {
    let Some(ref ncr) = form.value else {
        return Err(render_error(vec![
            "Path to source file should not be empty".to_string(),
        ]));
    };

    let message = Flash::success(
        Redirect::to(uri!(configure_clip)),
        ncr.file_path.trim().to_string(),
    );
    Ok(message)
}

#[post("/search", data = "<form>")]
pub async fn search_file(
    app: &State<App>,
    form: Form<Contextual<'_, models::SearchRequest>>,
) -> StdResult<Json<Vec<String>>, CustomStatus<String>> {
    let Some(search_engine) = &app.search else {
        return Err(CustomStatus(Status::Forbidden, format!(
            "Search is disabled because no source directory was specified in the {SEARCH_DIRS_VARNAME} env variable",
        )));
    };

    let Some(ref sr) = form.value else {
        return Err(CustomStatus(
            Status::BadRequest,
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
                .join("; "),
        ));
    };

    let search_fields = sr
        .search_string
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    if search_fields.is_empty() {
        return Err(CustomStatus(
            Status::BadRequest,
            "search fields should not be left empty".to_string(),
        ));
    }

    let results = search_engine.search(search_fields.as_slice());
    Ok(Json(results))
}

#[get("/refresh_index")]
pub async fn refresh_index(app: &State<App>) -> Template {
    let Some(search_engine) = &app.search else {
        return render_error(vec![format!(
            "Search is disabled because no source directory was specified in the {} env variable",
            SEARCH_DIRS_VARNAME
        )]);
    };
    if let Err(e) = search_engine.refresh_index() {
        render_error(vec![e.to_string()])
    } else {
        render_message("file index is being refreshed. This can take some time".to_string())
    }
}

#[get("/configure_clip")]
pub async fn configure_clip(app: &State<App>, flash: Option<FlashMessage<'_>>) -> Template {
    let source_file = flash.map_or_else(String::default, |msg| msg.message().to_string());
    if source_file.is_empty() {
        return render_error(vec!["Path to source file should not be empty".to_string()]);
    }
    match get_track_data(source_file.trim()) {
        Ok((at, st)) => Template::render(
            "configure",
            context! {app_name: &app.app_name, source_file, audio_tracks: at, subtitle_tracks: st},
        ),
        Err(e) => render_error(vec![format!("failed to get track data from file: {}", e)]),
    }
}

#[post("/configure_clip", data = "<form>")]
pub async fn create_clip(
    app: &State<App>,
    form: Form<Contextual<'_, models::ConfigureClipRequest>>,
) -> Template {
    let Some(ref ccr) = form.value else {
        return render_error(
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
        );
    };

    match setup_job(app, ccr) {
        Ok(()) => render_message(format!(
            "clip {} was successfully added to processing queue",
            ccr.clip_name
        )),
        Err(e) => render_error(vec![e.to_string()]),
    }
}

#[get("/delete?<file_name>")]
pub async fn delete_clip(file_name: String, app: &State<App>) -> Template {
    match clips_library::delete_file(&app.out_path, &file_name) {
        Ok(()) => render_message(format!("Clip {file_name} was successfully removed")),
        Err(e) => render_error(vec![format!("failed to remove file: {e}")]),
    }
}

#[get("/clear_failures")]
pub async fn clear_failures(app: &State<App>) -> Template {
    app.clipper.clear_failures();
    render_message("The list of failures was successfully cleared".to_string())
}

fn setup_job(app: &State<App>, ccr: &models::ConfigureClipRequest) -> AnyResult<()> {
    let (start_time, end_time) = validate_start_end(
        app.max_clip_duration,
        &ccr.start_hour,
        &ccr.start_min,
        &ccr.start_sec,
        &ccr.end_hour,
        &ccr.end_min,
        &ccr.end_sec,
    )?;

    let ext = if ccr.audio_only { "mp3" } else { "mp4" };

    let out_file_path = format!(
        "{}/{}.{}",
        app.out_path
            .to_str()
            .context("internal error: output path could not be read as string")?,
        ccr.clip_name,
        ext
    );

    app.clipper.add_job(Job::new(
        ccr.source_file.trim().to_string(),
        out_file_path,
        ccr.clip_name.to_string(),
        format!("{}.{}", ccr.clip_name, ext),
        ccr.audio_track.to_string(),
        ccr.subtitle_track.to_string(),
        start_time,
        end_time,
        ccr.audio_only,
    ))?;
    Ok(())
}

fn render_error(errors: Vec<String>) -> Template {
    Template::render("message", context! {errors})
}

fn render_message(message: String) -> Template {
    Template::render("message", context! {message})
}
