use crate::resp_to_res;
use crate::AppState;

use futures::future::TryFutureExt;
use gloo_net::http::Request;
use leptos::*;

#[component]
pub fn SearchResults(
    app_state_setter: WriteSignal<AppState>,
    search_string: String,
) -> impl IntoView {
    let (results_getter, results_setter) = create_signal(None);
    spawn_local(async move {
        let req = common::SearchRequest {
            search_string: search_string,
        };
        let clips = resp_to_res(
            Request::post("/search")
                .header("Content-Type", "application/json")
                .json(&req)
                .unwrap()
                .send(),
        )
        .and_then(|r| async move { Ok(r.json::<Vec<String>>().await?) })
        .await
        .ok();
        results_setter.set(clips)
    });

    view! {
        {move || if let Some(results) = results_getter.get().to_owned() {
            results.into_iter().map(
                move |file| {
                    let target_file = file.clone();
                    view!{
                    <form>
                        <button
                            class="secondary search-result"
                            on:click=move |_| app_state_setter.set(AppState::exact_path(target_file.clone()))>
                            {file.to_owned()}
                        </button>
                    </form>
                    }}).collect::<Vec<_>>()
                    } else {
                        vec![]
                    }}
    }
}
