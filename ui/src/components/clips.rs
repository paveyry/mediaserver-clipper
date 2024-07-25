use futures::future::TryFutureExt;
use gloo_net::http::Request;
use leptos::*;

#[component]
pub fn ClipsPanel() -> impl IntoView {
    view! {
        <p>Clips</p>
    }
}

// #[derive(Properties, PartialEq)]
// struct ClipsLibraryProp {
//     clips: Vec<common::ClipInfo>,
// }

// #[function_component(ClipsPanel)]
// pub fn clips_panel() -> Html {
//     let clips = use_state(|| None);
//     {
//         let clips = clips.clone();
//         use_effect_with((), move |_| {
//             let clips = clips.clone();
//             wasm_bindgen_futures::spawn_local(async move {
//                 let config = Request::get("/clips")
//                     .send()
//                     .and_then(|r| async move { r.json::<common::ClipsLibrary>().await })
//                     .await
//                     .ok();
//                 clips.set(config)
//             })
//         })
//     }
//     let Some(clips) = (*clips).clone() else {
//         return html! {};
//     };
//     let two_sections = !clips.audio.is_empty() && !clips.video.is_empty();
//     html! {
//         <>
//             <br/>
//             <div id="clips">
//                 if two_sections {
//                     <h3>{"Audio clips"}</h3>
//                 }
//                 <AudioClipsPanel clips={clips.audio}/>
//                 <br/>
//                 if two_sections {
//                     <h3>{"Video clips"}</h3>
//                 }
//                 <VideoClipsPanel clips={clips.video}/>
//             </div>
//         </>
//     }
// }

// #[function_component(VideoClipsPanel)]
// fn video_clips_panel(ClipsLibraryProp { clips }: &ClipsLibraryProp) -> Html {
//     clips
//         .chunks(2)
//         .map(|pair| {
//             html! {
//                 <>
//                     <div class="grid">
//                         <Video clip_info={pair[0].clone()}/>
//                         if let Some(ci) = pair.get(1) {
//                             <Video clip_info={ci.clone()}/>
//                         } else {
//                             <div></div>
//                         }
//                     </div>
//                     <br />
//                 </>
//             }
//         })
//         .collect()
// }

// #[function_component(AudioClipsPanel)]
// fn audio_clips_panel(ClipsLibraryProp { clips }: &ClipsLibraryProp) -> Html {
//     clips
//         .iter()
//         .map(|clip| {
//             html! {
//                 <div>
//                     <Audio clip_info={clip.clone()}/>
//                 </div>
//             }
//         })
//         .collect()
// }

// #[derive(Properties, PartialEq)]
// struct ClipProp {
//     clip_info: common::ClipInfo,
// }

// #[function_component(Audio)]
// fn audio(ClipProp { clip_info }: &ClipProp) -> Html {
//     let url = &clip_info.url;
//     let file_name = &clip_info.file_name;
//     let clip_name = &clip_info.clip_name;
//     let pub_url = &clip_info.public_url;
//     html! {
//         <div>
//             <label>{clip_name.to_owned()}{" "}
//                 <audio preload="none" controls=true><source src={url.to_owned()} type="audio/mp3"/></audio>
//                 <a href={url.to_owned()} download={file_name.to_owned()}><button class="outline primary"><i class="fa-solid fa-download"></i></button></a>
//                 <a href={url.to_owned()}><button class="outline primary"><i class="fa-solid fa-link"></i></button></a>
//                 <a href={pub_url.to_owned()}><button class="outline primary"><i class="fa-solid fa-share-nodes"></i></button></a>
//                 // <a href="/delete?file_name={{this.file_name}}"><button class="outline primary"><i class="fa-solid fa-trash"></i></button></a>
//             </label>
//         </div>
//     }
// }

// #[function_component(Video)]
// fn video(ClipProp { clip_info }: &ClipProp) -> Html {
//     let url = &clip_info.url;
//     let file_name = &clip_info.file_name;
//     let clip_name = &clip_info.clip_name;
//     let pub_url = &clip_info.public_url;
//     html! {
//         <div>
//             <div class="grid">
//                 <div>
//                     <a href={url.to_owned()} download={file_name.to_owned()}><button class="outline primary"><i class="fa-solid fa-download"></i></button></a>
//                     <a href={url.to_owned()}><button class="outline primary"><i class="fa-solid fa-link"></i></button></a>
//                     <a href={pub_url.to_owned()}><button class="outline primary"><i class="fa-solid fa-share-nodes"></i></button></a>
//                     // <a href="/delete?file_name={{this.0.file_name}}"><button class="outline primary"><i class="fa-solid fa-trash"></i></button></a>
//                 </div>
//                 <h2 class="video-title">{clip_name.to_owned()}</h2>
//             </div>
//             <video preload="none" controls=true>
//                 <source src={url.to_owned()} type="video/mp4"/>
//             </video>
//         </div>
//     }
// }
