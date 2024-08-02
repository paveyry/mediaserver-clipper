mod app; // App config and state
mod clipper; // Extracting clips from source video using ffmpeg
mod clips_library; // Clip library management
mod ffprobe; // Data extraction from source video files using ffprobe
mod routes; // Rocket routes
mod search; // Source file search engine

use crate::app::App;

use rocket::fs::{FileServer, Options};
use rocket::{launch, routes};

#[launch]
fn app() -> _ {
    let app = App::init();
    let output_dir = app.out_path.clone();
    rocket::build()
        .manage(app)
        .mount(
            common::constants::OUTPUT_ROUTE,
            FileServer::new(output_dir, Options::NormalizeDirs),
        )
        .mount(
            "/",
            routes![
                routes::root,
                routes::ui_files,
                routes::get_app_config,
                routes::get_clips,
                routes::get_ffprobe_tracks,
                routes::create_clip,
                routes::delete_clip,
                routes::search_files,
                routes::refresh_index,
                routes::index_refresh_status,
                routes::get_job_failures,
                routes::get_pending_jobs,
                routes::clear_job_failures,
            ],
        )
        .mount(
            "/static",
            FileServer::new("./static", Options::NormalizeDirs),
        )
}
