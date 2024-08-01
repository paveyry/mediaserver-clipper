mod components;

use components::clips::ClipsPanel;
use components::input::{ExactPathInput, SearchInput};
use components::search::SearchResults;

use futures::future::TryFutureExt;
use gloo_net::http::Request;
use leptos::*;
use log;

#[derive(Debug, Clone)]
enum AppState {
    ExactPath(String),
    Search(String),
    Home,
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
    let config = Request::get("/app_config")
        .send()
        .and_then(|r| async move { r.json::<common::Config>().await })
        .await
        .ok();
    if let Some(config) = config {
        signal.set(config);
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
            {move ||match app_state_getter.get() {
                AppState::ExactPath(path) => view!{<div><p> "PATH NOT IMPLEMENTED: " {path}</p></div>},
                AppState::Search(search_string) => view!{
                    <div>
                        <SearchInput callback=move |s| app_state_setter.set(AppState::Search(s)) />
                        <SearchResults app_state_setter=app_state_setter search_string=search_string />
                    </div>
                },
                AppState::Home => view!{
                    <div>
                        <ExactPathInput callback=move |s| app_state_setter.set(AppState::ExactPath(s)) />
                        <Show when=move || app_config_getter.get().search_enabled><SearchInput callback=move |s| app_state_setter.set(AppState::Search(s)) /></Show>
                        <ClipsPanel />
                    </div>
                },
            }}
        </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| view! { <App/> })
}
