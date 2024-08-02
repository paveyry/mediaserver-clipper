use crate::resp_to_res;
use crate::ErrorManager;

use futures::future::TryFutureExt;
use gloo_net::http::Request;
use leptos::*;

pub async fn update_clips_list(
    clips_setter: WriteSignal<Option<common::ClipsLibrary>>,
    errors_setter: WriteSignal<ErrorManager>,
) {
    let clips = resp_to_res(Request::get("/get_clips").send())
        .and_then(|r| async move { Ok(r.json::<common::ClipsLibrary>().await?) })
        .await;
    match clips {
        Ok(clips) => clips_setter.set(Some(clips)),
        Err(e) => {
            errors_setter.update(|s| s.add_err(format!("failed to retrieve clips library: {}", e)))
        }
    }
}

#[component]
pub fn ClipsPanel(
    clips_getter: ReadSignal<Option<common::ClipsLibrary>>,
    clips_setter: WriteSignal<Option<common::ClipsLibrary>>,
    errors_setter: WriteSignal<ErrorManager>,
) -> impl IntoView {
    spawn_local(async move {
        update_clips_list(clips_setter, errors_setter).await;
    });
    let two_sections = move || {
        let Some(clips) = clips_getter.get() else {
            return false;
        };
        !clips.audio.is_empty() && !clips.video.is_empty()
    };
    view! {
        <br/>
        <div id="clips">
            <Show when=two_sections><h3>"Audio clips"</h3></Show>
            <Show when=move || clips_getter.get().is_some()>
                <AudioClipsPanel clips=move|| clips_getter.get().unwrap().audio clips_setter errors_setter /> // TODO: avoid copy by splitting
                <br/>
                <Show when=two_sections><h3>{"Video clips"}</h3></Show>
                <VideoClipsPanel clips=move|| clips_getter.get().unwrap().video clips_setter errors_setter />
            </Show>
        </div>
    }
}

#[component]
fn VideoClipsPanel(
    #[prop(into)] clips: Signal<Vec<common::ClipInfo>>,
    clips_setter: WriteSignal<Option<common::ClipsLibrary>>,
    errors_setter: WriteSignal<ErrorManager>,
) -> impl IntoView {
    move || {
        clips
            .get()
            .chunks(2)
            .map(|pair| {
                let first = view! {
                        <div><Video clip_info=pair[0].to_owned() clips_setter errors_setter /></div>
                };
                let second = if pair.len() >= 2 {
                    view! {<div><Video clip_info=pair[1].to_owned() clips_setter errors_setter /></div>}
                } else {
                    view! {<div></div>}
                };
                view! {
                    <div class="grid">
                        {first}
                        {second}
                    </div>
                    <br />
                }
            })
            .collect_view()
    }
}

#[component]
fn AudioClipsPanel(
    #[prop(into)] clips: Signal<Vec<common::ClipInfo>>,
    clips_setter: WriteSignal<Option<common::ClipsLibrary>>,
    errors_setter: WriteSignal<ErrorManager>,
) -> impl IntoView {
    move || {
        clips
            .get()
            .iter()
            .map(|clip| {
                view! {
                    <div>
                        <Audio clip_info={clip.to_owned()} clips_setter errors_setter />
                    </div>
                }
            })
            .collect_view()
    }
}

fn delete_clip(
    clip_name: String,
    clips_setter: WriteSignal<Option<common::ClipsLibrary>>,
    errors_setter: WriteSignal<ErrorManager>,
) {
    spawn_local(async move {
        let req = common::DeleteClipRequest {
            clip_name: clip_name.clone(),
        };
        let res = resp_to_res(
            Request::post("/delete_clip")
                .header("Content-Type", "application/json")
                .json(&req)
                .unwrap()
                .send(),
        )
        .await;
        match res {
            Ok(_) => spawn_local(async move {
                update_clips_list(clips_setter, errors_setter).await;
            }),
            Err(e) => errors_setter
                .update(|s| s.add_err(format!("failed to delete clip {clip_name}: {e}"))),
        }
    });
}

#[component]
fn Audio(
    clip_info: common::ClipInfo,
    clips_setter: WriteSignal<Option<common::ClipsLibrary>>,
    errors_setter: WriteSignal<ErrorManager>,
) -> impl IntoView {
    let url = &clip_info.url;
    let file_name = &clip_info.file_name;
    let clip_name = &clip_info.clip_name;
    let pub_url = &clip_info.public_url;
    let del = {
        let file_name = file_name.clone();
        move |_| delete_clip(file_name.to_owned(), clips_setter, errors_setter)
    };
    view! {
        <label>{clip_name.to_owned()}{" "}
            <audio preload="none" controls=true><source src={url.to_owned()} type="audio/mp3"/></audio>
            <a href={url.to_owned()} download={file_name.to_owned()}><button class="outline primary"><i class="fa-solid fa-download"></i></button></a>
            <a href={url.to_owned()}><button class="outline primary"><i class="fa-solid fa-link"></i></button></a>
            <a href={pub_url.to_owned()}><button class="outline primary"><i class="fa-solid fa-share-nodes"></i></button></a>
            <button on:click=del class="outline primary"><i class="fa-solid fa-trash"></i></button>
        </label>
    }
}

#[component]
fn Video(
    clip_info: common::ClipInfo,
    clips_setter: WriteSignal<Option<common::ClipsLibrary>>,
    errors_setter: WriteSignal<ErrorManager>,
) -> impl IntoView {
    let url = &clip_info.url;
    let file_name = &clip_info.file_name;
    let clip_name = &clip_info.clip_name;
    let pub_url = &clip_info.public_url;
    let del = {
        let file_name = file_name.clone();
        move |_| delete_clip(file_name.to_owned(), clips_setter, errors_setter)
    };
    view! {
        <div class="grid">
            <div>
                <a href={url.to_owned()} download={file_name.to_owned()}><button class="outline primary"><i class="fa-solid fa-download"></i></button></a>
                <a href={url.to_owned()}><button class="outline primary"><i class="fa-solid fa-link"></i></button></a>
                <a href={pub_url.to_owned()}><button class="outline primary"><i class="fa-solid fa-share-nodes"></i></button></a>
                <button on:click=del class="outline primary"><i class="fa-solid fa-trash"></i></button>
            </div>
            <h2 class="video-title">{clip_name.to_owned()}</h2>
        </div>
        <video preload="none" controls=true>
            <source src={url.to_owned()} type="video/mp4"/>
        </video>
    }
}
