use crate::components::clips::ClipsPanel;
use crate::components::input::{ExactPathInput, SearchInput};
use crate::components::status::{FailedClips, IndexRefreshing, PendingJobs};
use crate::{AppState, ErrorManager};

use common::Config as AppConfig;

use leptos::*;

#[component]
pub fn HomePage(
    app_config_getter: ReadSignal<AppConfig>,
    #[prop(into)] app_state_setter: Callback<AppState>,
    errors_setter: WriteSignal<ErrorManager>,
) -> impl IntoView {
    let (clips_getter, clips_setter) = create_signal(None);
    let (failures_getter, failures_setter) = create_signal(Vec::default());

    view! {
        <ExactPathInput callback=move |s: String| app_state_setter.call(AppState::ExactPath(s)) />
        <Show when=move || app_config_getter.get().search_enabled><SearchInput callback=move |s| app_state_setter.call(AppState::Search(s)) /></Show>
        <IndexRefreshing errors_setter />
        <PendingJobs clips_setter errors_setter failures_setter />
        <FailedClips failures_getter failures_setter errors_setter />
        <ClipsPanel clips_getter clips_setter errors_setter />
    }
}
