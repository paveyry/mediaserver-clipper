mod components;

use components::clips::ClipsPanel;
use components::input::{ExactPathInput, SearchInput};
// use components::input::{FileInput, SearchInput};

use std::marker::PhantomData;
use std::thread::spawn;

use futures::future::TryFutureExt;
use gloo_net::http::Request;
use leptos::*;
use log;
// use yew::prelude::*;

// #[function_component(App)]
// fn app() -> Html {
//     let app_config = use_state(|| None);
//     {
//         let app_config = app_config.clone();
//         use_effect_with((), move |_| {
//             let app_config = app_config.clone();
//             wasm_bindgen_futures::spawn_local(async move {
//                 let config = Request::get("/app_config")
//                     .send()
//                     .and_then(|r| async move { r.json::<common::Config>().await })
//                     .await
//                     .ok();
//                 app_config.set(config)
//             })
//         })
//     }

//     log::info!("FOOBAR:{:?}", (*app_config).clone());

//     html! {
//         <main class="container">
//             <a href="/"><h1>{ (*app_config).clone().map_or_else(String::default, |ac| ac.app_name) }</h1></a>

//             <FileInput />
//             <SearchInput />
//             <ClipsPanel />
//         </main>
//     }
// }

#[derive(Debug, Clone)]
enum AppState {
    ExactPath(String),
    Search(String),
    Home,
}

impl std::fmt::Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Home
    }
}

#[component]
fn ProgressBar(#[prop(into)] progress: Signal<i32>, //impl Fn() -> i32 + 'static
) -> impl IntoView {
    view! {
        <progress
            max="50"
            // now this works
            value=progress
        />
    }
}

async fn get_app_config(signal: WriteSignal<common::Config>) {
    log::info!("getting");
    let config = Request::get("/app_config")
        .send()
        .and_then(|r| async move { r.json::<common::Config>().await })
        .await
        .ok();
    log::info!("got");
    if let Some(config) = config {
        log::info!("data: {:?}", &config);
        // signal.set(config);
    }
    // TODO: Handle error
}

#[component]
fn App() -> impl IntoView {
    let (app_state_getter, app_state_setter) = create_signal(AppState::default());
    let (app_config_getter, app_config_setter) = create_signal(common::Config::default());
    app_state_getter.with(|s| log::info!("WITH HOOK: {s:?}"));

    spawn_local(async move {
        get_app_config(app_config_setter).await;
    });

    view! {
        <main class="container">
            <h1 class="apptitle" on:click=move |_| app_state_setter.set(AppState::Home)>{move || app_config_getter.get().app_name}</h1>
            <Show when=move || !matches!(app_state_getter.get(), AppState::Home)>
                <p> "PAGE NOT IMPLEMENTED: " {move || app_state_getter.get().to_string()}</p>
            </Show>
            <Show when=move || matches!(app_state_getter.get(), AppState::Home)>
                <ExactPathInput callback=move |s| app_state_setter.set(AppState::ExactPath(s)) />
                <SearchInput callback=move |s| app_state_setter.set(AppState::Search(s)) />
            </Show>
        </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| view! { <App/> })
}
