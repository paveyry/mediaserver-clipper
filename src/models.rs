use rocket::FromForm;

#[derive(FromForm, Debug)]
pub struct NewClipRequest {
    #[field(validate=len(1..))]
    pub(crate) file_path: String,
}

#[derive(FromForm, Debug)]
pub struct SearchRequest {
    #[field(validate=len(1..))]
    pub(crate) search_string: String,
}

#[derive(FromForm, Debug)]
pub struct ConfigureClipRequest {
    #[field(validate=len(1..))]
    pub(crate) source_file: String,
    #[field(validate=len(1..))]
    pub(crate) clip_name: String,
    #[field(validate=len(1..))]
    pub(crate) audio_track: String,
    #[field(validate=len(0..))]
    pub(crate) subtitle_track: String,
    #[field(validate=len(0..))]
    pub(crate) start_sec: String,
    #[field(validate=len(0..))]
    pub(crate) start_min: String,
    #[field(validate=len(0..))]
    pub(crate) start_hour: String,
    #[field(validate=len(0..))]
    pub(crate) end_sec: String,
    #[field(validate=len(0..))]
    pub(crate) end_min: String,
    #[field(validate=len(0..))]
    pub(crate) end_hour: String,
    #[field(default = false)]
    pub(crate) audio_only: bool,
}
