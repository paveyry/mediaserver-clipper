mod components;

use std::future::Future;

use components::clips::ClipsPanel;
use components::create::ClipCreationForm;
use components::input::{ExactPathInput, SearchInput};
use components::search::SearchResults;

use anyhow::Context;
use futures::future::TryFutureExt;
use gloo_net::http::{Request, Response};
use leptos::*;
use log;

pub async fn resp_to_res(
    r: impl Future<Output = Result<Response, gloo_net::Error>>,
) -> Result<Response, anyhow::Error> {
    let resp = r.await?;
    if resp.ok() {
        Ok(resp)
    } else {
        Err(anyhow::Error::msg(format!(
            "{} {}: {}",
            resp.status(),
            resp.status_text(),
            resp.text().await?
        )))
    }
}

#[derive(Default, Debug, Clone)]
struct AppState {
    pub route: AppRoute,
    pub error: String,
}

impl AppState {
    fn home() -> Self {
        Self::default()
    }

    fn exact_path(path: String) -> Self {
        Self {
            route: AppRoute::ExactPath(path),
            ..Default::default()
        }
    }

    fn search(search_string: String) -> Self {
        Self {
            route: AppRoute::Search(search_string),
            ..Default::default()
        }
    }

    fn err(self, e: String) -> Self {
        Self {
            route: self.route,
            error: e,
        }
    }

    fn set_err(&mut self, e: String) {
        self.error = e
    }
}

#[derive(Debug, Clone)]
enum AppRoute {
    ExactPath(String),
    Search(String),
    Home,
}

impl AppRoute {
    fn local_title(&self) -> &'static str {
        match self {
            Self::ExactPath(_) => " - Create a new clip",
            Self::Search(_) => " - Search results",
            Self::Home => "",
        }
    }
}

impl Default for AppRoute {
    fn default() -> Self {
        Self::Home
    }
}

async fn get_app_config(
    config_setter: WriteSignal<common::Config>,
    state_setter: WriteSignal<AppState>,
) {
    let config = resp_to_res(Request::get("/app_config").send())
        .and_then(|r| async move { Ok(r.json::<common::Config>().await?) })
        .await;
    match config {
        Ok(config) => config_setter.set(config),
        Err(e) => state_setter.update(|s| s.set_err(format!("failed to retrieve App Config: {e}"))),
    }
}

#[component]
fn App() -> impl IntoView {
    let (app_state_getter, app_state_setter) = create_signal(AppState::default());
    let (app_config_getter, app_config_setter) = create_signal(common::Config::default());
    app_state_getter.with(|s| log::info!("WITH HOOK: {s:?}"));

    spawn_local(async move {
        get_app_config(app_config_setter, app_state_setter).await;
    });

    view! {
        <main class="container">
            <h1 class="apptitle" on:click=move |_| app_state_setter.set(AppState::home())>{move || app_config_getter.get().app_name}{move || app_state_getter.get().route.local_title()}</h1>
            <Show when=move || !app_state_getter.get().error.is_empty()>
                <article class="error-message">{move || app_state_getter.get().error}</article>
            </Show>
            {move || match app_state_getter.get().route {
                AppRoute::ExactPath(path) => view!{
                    <div>
                        <ClipCreationForm app_state_setter=app_state_setter file_path=path />
                    </div>
                },
                AppRoute::Search(search_string) => view!{
                    <div>
                        <SearchInput callback=move |s| app_state_setter.set(AppState::search(s)) />
                        <SearchResults app_state_setter=app_state_setter search_string=search_string />
                    </div>
                },
                AppRoute::Home => view!{
                    <div>
                        <ExactPathInput callback=move |s| app_state_setter.set(AppState::exact_path(s)) />
                        <Show when=move || app_config_getter.get().search_enabled><SearchInput callback=move |s| app_state_setter.set(AppState::search(s)) /></Show>
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
