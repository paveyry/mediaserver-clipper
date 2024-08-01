use futures::future::TryFutureExt;
use gloo_net::http::Request;
use leptos::*;

#[component]
pub fn ClipsPanel() -> impl IntoView {
    let (clips_getter, clips_setter) = create_signal(None);
    spawn_local(async move {
        let clips = Request::get("/clips")
            .send()
            .and_then(|r| async move { r.json::<common::ClipsLibrary>().await })
            .await
            .ok();
        clips_setter.set(clips)
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
                <AudioClipsPanel clips=move || clips_getter.get().unwrap().audio/>
                <br/>
                <Show when=two_sections><h3>{"Video clips"}</h3></Show>
                <VideoClipsPanel clips=move || clips_getter.get().unwrap().video/>
            </Show>
        </div>
    }
}

#[component]
fn VideoClipsPanel(#[prop(into)] clips: Signal<Vec<common::ClipInfo>>) -> impl IntoView {
    let clip_list = clips.get().clone();
    clip_list
        .chunks(2)
        .map(|pair| {
            let first = view! {
                    <div><Video clip_info=pair[0].to_owned() /></div>
            };
            let second = if pair.len() >= 2 {
                view! {<div><Video clip_info=pair[1].to_owned() /></div>}
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
        .collect::<Vec<_>>()
}

#[component]
fn AudioClipsPanel(#[prop(into)] clips: Signal<Vec<common::ClipInfo>>) -> impl IntoView {
    clips
        .get()
        .iter()
        .map(|clip| {
            view! {
                <div>
                    <Audio clip_info={clip.to_owned()}/>
                </div>
            }
        })
        .collect::<Vec<_>>()
}

#[component]
fn Audio(clip_info: common::ClipInfo) -> impl IntoView {
    let url = &clip_info.url;
    let file_name = &clip_info.file_name;
    let clip_name = &clip_info.clip_name;
    let pub_url = &clip_info.public_url;
    view! {
        <label>{clip_name.to_owned()}{" "}
            <audio preload="none" controls=true><source src={url.to_owned()} type="audio/mp3"/></audio>
            <a href={url.to_owned()} download={file_name.to_owned()}><button class="outline primary"><i class="fa-solid fa-download"></i></button></a>
            <a href={url.to_owned()}><button class="outline primary"><i class="fa-solid fa-link"></i></button></a>
            <a href={pub_url.to_owned()}><button class="outline primary"><i class="fa-solid fa-share-nodes"></i></button></a>
            // <a href="/delete?file_name={{this.file_name}}"><button class="outline primary"><i class="fa-solid fa-trash"></i></button></a>
        </label>
    }
}

#[component]
fn Video(clip_info: common::ClipInfo) -> impl IntoView {
    let url = &clip_info.url;
    let file_name = &clip_info.file_name;
    let clip_name = &clip_info.clip_name;
    let pub_url = &clip_info.public_url;
    view! {
        <div class="grid">
            <div>
                <a href={url.to_owned()} download={file_name.to_owned()}><button class="outline primary"><i class="fa-solid fa-download"></i></button></a>
                <a href={url.to_owned()}><button class="outline primary"><i class="fa-solid fa-link"></i></button></a>
                <a href={pub_url.to_owned()}><button class="outline primary"><i class="fa-solid fa-share-nodes"></i></button></a>
                // <a href="/delete?file_name={{this.0.file_name}}"><button class="outline primary"><i class="fa-solid fa-trash"></i></button></a>
            </div>
            <h2 class="video-title">{clip_name.to_owned()}</h2>
        </div>
        <video preload="none" controls=true>
            <source src={url.to_owned()} type="video/mp4"/>
        </video>
    }
}
