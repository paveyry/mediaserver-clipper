use crate::app::{App, SEARCH_DIRS_VARNAME};
use crate::clipper::{validate_start_end, Job};
use crate::clips;
use crate::ffprobe::get_track_data;
use crate::models;

use anyhow::{Context, Result};
use rocket::form::{Contextual, Form};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket::{get, post, uri};
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub async fn root(app: &State<App>) -> Template {
    let (search_enabled, index_refreshing) = app
        .search
        .as_ref()
        .map_or((false, false), |index| (true, index.is_refreshing()));

    let pending = app.clipper.jobs_in_progress();

    match clips::video_pairs_in_directory(&app.out_path, &app.public_link_prefix, &pending) {
        Ok(clips) => Template::render(
            "root",
            context! { app_name: &app.app_name, clips: clips, pending_jobs: pending, index_refreshing, search_enabled},
        ),
        Err(e) => render_error(vec![format!(
            "Failed to list clips from output directory: {}",
            e
        )]),
    }
}

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
) -> Template {
    let Some(search_engine) = &app.search else {
        return render_error(vec![format!(
            "Search is disabled because no source directory was specified in the {} env variable",
            SEARCH_DIRS_VARNAME
        )]);
    };

    let Some(ref sr) = form.value else {
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

    let search_fields = sr
        .search_string
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    if search_fields.is_empty() {
        return render_error(vec!["search fields should not be left empty".to_string()]);
    }

    let results = search_engine.search(search_fields.as_slice());
    Template::render(
        "search_result",
        context! { app_name: &app.app_name, results},
    )
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

#[get("/delete?<clip_name>")]
pub async fn delete_clip(clip_name: String, app: &State<App>) -> Template {
    match clips::delete_file(&app.out_path, &clip_name) {
        Ok(()) => render_message(format!("Clip {clip_name} was successfully removed")),
        Err(e) => render_error(vec![format!("failed to remove file: {e}")]),
    }
}

fn setup_job(app: &State<App>, ccr: &models::ConfigureClipRequest) -> Result<()> {
    let (start_time, end_time) = validate_start_end(
        app.max_clip_duration,
        &ccr.start_hour,
        &ccr.start_min,
        &ccr.start_sec,
        &ccr.end_hour,
        &ccr.end_min,
        &ccr.end_sec,
    )?;

    let out_file_path = format!(
        "{}/{}.mp4",
        app.out_path
            .to_str()
            .context("internal error: output path could not be read as string")?,
        ccr.clip_name
    );

    app.clipper.add_job(Job::new(
        ccr.source_file.trim().to_string(),
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