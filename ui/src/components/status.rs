use std::collections::HashSet;
use std::time::Duration;

use crate::components::clips;
use crate::resp_to_res;
use crate::ErrorManager;

use futures::future::TryFutureExt;
use gloo_net::http::Request;
use leptos::*;

async fn update_index_refresh_status(
    refreshing_setter: WriteSignal<bool>,
    errors_setter: WriteSignal<ErrorManager>,
) {
    let refreshing = resp_to_res(Request::get("/is_index_refreshing").send())
        .and_then(|r| async move { Ok(r.json::<bool>().await?) })
        .await;
    match refreshing {
        Ok(refreshing) => {
            refreshing_setter.set(refreshing);
            if refreshing {
                set_timeout(
                    move || {
                        spawn_local(async move {
                            update_index_refresh_status(refreshing_setter, errors_setter).await;
                        })
                    },
                    Duration::new(3, 0),
                );
            }
        }
        Err(e) => errors_setter.update(|s| s.add_err(e.to_string())),
    }
}

#[component]
pub fn IndexRefreshing(errors_setter: WriteSignal<ErrorManager>) -> impl IntoView {
    let (refreshing_getter, refreshing_setter) = create_signal(false);
    spawn_local(async move {
        update_index_refresh_status(refreshing_setter, errors_setter).await;
    });
    view! {
        <Show when=move || refreshing_getter.get()>
            <article id="index_refreshing">
                <p><strong>File index for search is currently being updated.</strong></p>
            </article>
        </Show>
    }
}

async fn update_pending_jobs(
    old: HashSet<String>,
    pending_setter: WriteSignal<HashSet<String>>,
    errors_setter: WriteSignal<ErrorManager>,
    clips_setter: WriteSignal<Option<common::ClipsLibrary>>,
    failures_setter: WriteSignal<Vec<String>>,
) {
    let pending = resp_to_res(Request::get("/get_pending_jobs").send())
        .and_then(|r| async move { Ok(r.json::<HashSet<String>>().await?) })
        .await;
    match pending {
        Ok(pending) => {
            pending_setter.set(pending.clone());
            if pending != old {
                // if the set of pending clips has changed, refresh clip list and failures list
                spawn_local(async move {
                    clips::update_clips_list(clips_setter, errors_setter).await;
                });
                spawn_local(async move {
                    update_failures(failures_setter, errors_setter).await;
                });
            }
            if pending.is_empty() {
                return;
            }
            set_timeout(
                move || {
                    spawn_local(async move {
                        update_pending_jobs(
                            pending,
                            pending_setter,
                            errors_setter,
                            clips_setter,
                            failures_setter,
                        )
                        .await;
                    })
                },
                Duration::new(5, 0),
            );
        }
        Err(e) => errors_setter.update(|s| s.add_err(e.to_string())),
    }
}

#[component]
pub fn PendingJobs(
    errors_setter: WriteSignal<ErrorManager>,
    clips_setter: WriteSignal<Option<common::ClipsLibrary>>,
    failures_setter: WriteSignal<Vec<String>>,
) -> impl IntoView {
    let (pending_getter, pending_setter) = create_signal(HashSet::default());
    spawn_local(async move {
        update_pending_jobs(
            HashSet::default(),
            pending_setter,
            errors_setter,
            clips_setter,
            failures_setter,
        )
        .await;
    });
    view! {
        <Show when=move || !pending_getter.get().is_empty()>
            <article id="pending_jobs">
                <h5>Clips in queue</h5>
                <ul>
                    {move || pending_getter.get().into_iter().map(|j| view!{<li>{j}</li>}).collect_view()}
                </ul>
            </article>
        </Show>
    }
}

async fn update_failures(
    failures_setter: WriteSignal<Vec<String>>,
    errors_setter: WriteSignal<ErrorManager>,
) {
    let failures = resp_to_res(Request::get("/get_job_failures").send())
        .and_then(|r| async move { Ok(r.json::<Vec<String>>().await?) })
        .await;
    match failures {
        Ok(failures) => failures_setter.set(failures),
        Err(e) => errors_setter.update(|s| s.add_err(e.to_string())),
    }
}

#[component]
pub fn FailedClips(
    failures_getter: ReadSignal<Vec<String>>,
    failures_setter: WriteSignal<Vec<String>>,
    errors_setter: WriteSignal<ErrorManager>,
) -> impl IntoView {
    spawn_local(async move {
        update_failures(failures_setter, errors_setter).await;
    });
    let clear = move |_| {
        spawn_local(async move {
            let res = resp_to_res(Request::get("/clear_job_failures").send()).await;
            if let Err(e) = res {
                errors_setter.update(|s| s.add_err(e.to_string()));
                return;
            }
            spawn_local(async move {
                update_failures(failures_setter, errors_setter).await;
            })
        })
    };

    view! {
        <Show when=move || !failures_getter.get().is_empty()>
            <article id="failures">
                <h5>Failed clips</h5>
                <ul>
                    {move || failures_getter.get().into_iter().map(|f| view!{<li>{f}</li>}).collect_view()}
                </ul>
                <button on:click=clear>Clear failures</button>
            </article>
        </Show>
    }
}
