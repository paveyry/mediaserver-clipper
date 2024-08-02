use crate::resp_to_res;
use crate::AppState;
use common::{ConfigureClipRequest, FFProbeResult};

use futures::future::TryFutureExt;
use gloo_net::http::Request;
use leptos::*;

#[component]
pub fn ClipCreationForm(
    app_state_setter: WriteSignal<AppState>,
    file_path: String,
) -> impl IntoView {
    let (tracks_info_getter, tracks_info_setter) = create_signal(None);
    {
        let file_path = file_path.clone();
        spawn_local(async move {
            let req = common::FFProbeRequest { file_path };
            let tracks_info = resp_to_res(
                Request::post("/ffprobe")
                    .header("Content-Type", "application/json")
                    .json(&req)
                    .unwrap()
                    .send(),
            )
            .and_then(|r| async move { Ok(r.json::<FFProbeResult>().await?) })
            .await;
            match tracks_info {
                Ok(tracks_info) => tracks_info_setter.set(Some(tracks_info)),
                Err(e) => app_state_setter.set(AppState::home().err(e.to_string())),
            }
        });
    }

    let start_hour: NodeRef<html::Input> = create_node_ref();
    let start_min: NodeRef<html::Input> = create_node_ref();
    let start_sec: NodeRef<html::Input> = create_node_ref();
    let end_hour: NodeRef<html::Input> = create_node_ref();
    let end_min: NodeRef<html::Input> = create_node_ref();
    let end_sec: NodeRef<html::Input> = create_node_ref();
    let clip_name: NodeRef<html::Input> = create_node_ref();
    let audio_track: NodeRef<html::Select> = create_node_ref();
    let sub_track: NodeRef<html::Select> = create_node_ref();
    let audio_only_toggle: NodeRef<html::Input> = create_node_ref();

    let on_submit = {
        let file_path = file_path.clone();
        move |ev: leptos::ev::SubmitEvent| {
            ev.prevent_default();
            let req = ConfigureClipRequest {
                source_file: file_path.clone(),
                clip_name: clip_name.get().map(|e| e.value()).unwrap(),
                audio_track: audio_track.get().map(|e| e.value()).unwrap(),
                subtitle_track: sub_track.get().map(|e| e.value()).unwrap_or_default(),
                start_sec: start_sec.get().map(|e| e.value()).unwrap_or_default(),
                start_min: start_min.get().map(|e| e.value()).unwrap_or_default(),
                start_hour: start_hour.get().map(|e| e.value()).unwrap_or_default(),
                end_sec: end_sec.get().map(|e| e.value()).unwrap_or_default(),
                end_min: end_min.get().map(|e| e.value()).unwrap_or_default(),
                end_hour: end_hour.get().map(|e| e.value()).unwrap_or_default(),
                audio_only: audio_only_toggle
                    .get()
                    .map(|e| e.checked())
                    .unwrap_or_default(),
            };
            spawn_local(async move {
                let resp = resp_to_res(
                    Request::post("/create_clip")
                        .header("Content-Type", "application/json")
                        .json(&req)
                        .unwrap()
                        .send(),
                )
                .await;
                match resp {
                    Ok(_) => app_state_setter.set(AppState::home()),
                    Err(e) => app_state_setter.update(|s| s.set_err(e.to_string())),
                }
            });
        }
    };

    view! {
        <form on:submit=on_submit>
            <label>Source file</label>
            <input name="source_file" value=file_path.clone() disabled=true />
            <label>Clip name</label>
            <input type="text" name="clip_name" required=true node_ref=clip_name/>
            <div class="grid">
                <div>
                    <label>Start time</label>
                    <fieldset role="group">
                        <input type="text"
                            name="start_hour"
                            placeholder="h"
                            node_ref=start_hour
                        />
                        <input type="text"
                            name="start_min"
                            placeholder="m"
                            node_ref=start_min
                        />
                        <input type="text"
                            name="start_sec"
                            placeholder="s"
                            node_ref=start_sec
                        />
                    </fieldset>
                    <label>End time</label>
                    <fieldset role="group">
                        <input type="text"
                            name="end_hour"
                            placeholder="h"
                            node_ref=end_hour
                        />
                        <input type="text"
                            name="end_min"
                            placeholder="m"
                            node_ref=end_min
                        />
                        <input type="text"
                            name="end_sec"
                            placeholder="s"
                            node_ref=end_sec
                        />
                    </fieldset>
                </div>
                <div>
                    <label>Audio track</label>
                    <select name="audio_track" required=true node_ref=audio_track>
                        {
                            move || tracks_info_getter.get().unwrap_or_default().audio_tracks.iter().map(
                                |t| view!{<option value=t.clone()>{t.clone()}</option>}).collect_view()
                        }
                    </select>
                    <label>Subtitle track</label>
                    <select name="subtitle_track" node_ref=sub_track>
                        {
                            move || tracks_info_getter.get().unwrap_or_default().sub_tracks.iter().map(
                                |t| view!{<option value=t.clone()>{t.clone()}</option>}).collect_view()
                        }
                    </select>
                </div>
                <div>
                    <label>Audio only (mp3)</label>
                    <input type="checkbox" name="audio_only" role="switch" node_ref=audio_only_toggle/>
                </div>
                <div></div>
                <div></div>
            </div>
                <button type="submit">Make a clip!</button>
        </form>
    }
}
