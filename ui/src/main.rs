mod components;

use components::clips::ClipsPanel;
use components::input::{FileInput, SearchInput};

use futures::future::TryFutureExt;
use gloo_net::http::Request;
use log;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let app_config = use_state(|| None);
    {
        let app_config = app_config.clone();
        use_effect_with((), move |_| {
            let app_config = app_config.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let config = Request::get("/app_config")
                    .send()
                    .and_then(|r| async move { r.json::<common::Config>().await })
                    .await
                    .ok();
                app_config.set(config)
            })
        })
    }

    log::info!("FOOBAR:{:?}", (*app_config).clone());

    html! {
        <main class="container">
            <a href="/"><h1>{ (*app_config).clone().map_or_else(String::default, |ac| ac.app_name) }</h1></a>

            <FileInput />
            <SearchInput />
            <ClipsPanel />
        </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
