mod app; // App config and state
mod clipper; // Extracting clips from source video using ffmpeg
mod clips_library; // Clip library management
mod ffprobe; // Data extraction from source video files using ffprobe
mod routes; // Rocket routes
mod search; // Source file search engine

use crate::app::App;

use rocket::fs::{FileServer, Options};
use rocket::{launch, routes};
use rocket_dyn_templates::Template;

#[launch]
fn app() -> _ {
    let app = App::init();
    let output_dir = app.out_path.clone();
    rocket::build()
        .attach(Template::fairing())
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
                routes::app_config,
                routes::clips,
                routes::delete_clip,
                routes::create_clip,
                routes::select_source,
                routes::search_file,
                routes::refresh_index,
                routes::clear_failures,
            ],
        )
        .mount(
            "/static",
            FileServer::new("./static", Options::NormalizeDirs),
        )
}
