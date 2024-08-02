use crate::components::input::SearchInput;
use crate::resp_to_res;
use crate::{AppState, ErrorManager};

use futures::future::TryFutureExt;
use gloo_net::http::Request;
use leptos::*;

#[component]
pub fn SearchResults(
    #[prop(into)] app_state_setter: Callback<AppState>,
    errors_setter: WriteSignal<ErrorManager>,
    search_string: String,
) -> impl IntoView {
    let (results_getter, results_setter) = create_signal(None);
    spawn_local(async move {
        let req = common::SearchRequest { search_string };
        let clips = resp_to_res(
            Request::post("/search_files")
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

    let refresh = move |_| {
        spawn_local(async move {
            let res = resp_to_res(Request::get("/refresh_index").send()).await;
            app_state_setter.call(AppState::Home);
            if let Err(e) = res {
                errors_setter.update(|s| s.add_err(e.to_string()));
            }
        })
    };

    view! {
        <button on:click=refresh class="secondary"><i class="fa-solid fa-screwdriver-wrench"></i> File is missing because it was recently added? Refresh the file index</button>
        <br/>
        <br/>
        <SearchInput callback=move |s| app_state_setter.call(AppState::Search(s)) />

        {move || if let Some(results) = results_getter.get().to_owned() {
            results.into_iter().map(
                move |file| {
                    let target_file = file.clone();
                    view!{
                    <form>
                        <button
                            class="secondary search-result"
                            on:click=move |_| app_state_setter.call(AppState::ExactPath(target_file.clone()))>
                            {file.to_owned()}
                        </button>
                    </form>
                    }}).collect::<Vec<_>>()
                    } else {
                        vec![]
                    }}
    }
}
