mod components;

use std::future::Future;

use components::create::ClipCreationForm;
use components::home::HomePage;
use components::search::SearchResults;

use futures::future::TryFutureExt;
use gloo_net::http::{Request, Response};
use leptos::*;

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
struct ErrorManager(Vec<String>);

impl ErrorManager {
    fn add_err(&mut self, e: String) {
        if self.0.len() >= 5 {
            for err in &self.0 {
                log::error!("{err}");
            }
            self.0 = vec!["Too many errors, check console".to_string()];
            return;
        }
        self.0.push(e);
    }

    fn error_msg(&self) -> impl IntoView {
        if self.0.is_empty() {
            return view! {<div></div>};
        }
        let errors = self.0.clone();
        view! {
            <div>
                <ul>
                { move || errors.clone().into_iter().map(|e| view!{<li>{e}</li>}).collect_view() }
                </ul>
            </div>
        }
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug, Clone)]
enum AppState {
    ExactPath(String),
    Search(String),
    Home,
}

impl AppState {
    fn local_title(&self) -> &'static str {
        match self {
            Self::ExactPath(_) => " - Create a new clip",
            Self::Search(_) => " - Search results",
            Self::Home => "",
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Home
    }
}

async fn get_app_config(
    config_setter: WriteSignal<common::Config>,
    error_setter: WriteSignal<ErrorManager>,
) {
    let config = resp_to_res(Request::get("/get_app_config").send())
        .and_then(|r| async move { Ok(r.json::<common::Config>().await?) })
        .await;
    match config {
        Ok(config) => config_setter.set(config),
        Err(e) => error_setter.update(|s| s.add_err(format!("failed to retrieve App Config: {e}"))),
    }
}

#[component]
fn App() -> impl IntoView {
    let (app_state_getter, app_state_setter) = create_signal(AppState::default());
    let (errors_getter, errors_setter) = create_signal(ErrorManager::default());
    let (app_config_getter, app_config_setter) = create_signal(common::Config::default());
    let app_state_setter = {
        move |v: AppState| {
            errors_setter.set(ErrorManager::default());
            app_state_setter.set(v);
        }
    };

    spawn_local(async move {
        get_app_config(app_config_setter, errors_setter).await;
    });

    view! {
        <main class="container">
            <h1 class="apptitle" on:click=move |_| app_state_setter(AppState::Home)>{move || app_config_getter.get().app_name}{move || app_state_getter.get().local_title()}</h1>
            <Show when=move || !errors_getter.get().is_empty()>
                <article class="error-message">{move || errors_getter.get().error_msg()}</article>
            </Show>
            {move || match app_state_getter.get() {
                AppState::ExactPath(path) => view!{
                    <div>
                        <ClipCreationForm app_state_setter errors_setter file_path=path />
                    </div>
                },
                AppState::Search(search_string) => view!{
                    <div>
                        <SearchResults app_state_setter errors_setter search_string=search_string />
                    </div>
                },
                AppState::Home => view!{
                    <div>
                        <HomePage app_config_getter app_state_setter errors_setter />
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
