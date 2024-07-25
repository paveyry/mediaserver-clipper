use yew::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

fn single_text_form(state: UseStateHandle<Option<String>>, name: &str, placeholder: &str, button_text: &str, callback: Callback<Option<String>>) -> Html {
    let on_text_changed = {
        let state = state.clone();
        Callback::from(move |event: Event| {
            let target = event.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok().and_then(|t| Some(t.value())));
            state.set(target);
        })
    };

    let on_submit = {
        let state = state.clone();
        Callback::from(move |event: SubmitEvent| {
            callback.emit((*state).clone());
            event.prevent_default();
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <fieldset role="group">
                <input onchange={on_text_changed} type="text"
                    name={name.to_string()}
                    placeholder={placeholder.to_string()}
                    aria-label={placeholder.to_string()}
                    required=true
                />
                <button>{button_text}</button>
            </fieldset>
        </form>
    }
}

#[function_component(FileInput)]
pub fn file_input() -> Html {
    let state = use_state(|| None);
    single_text_form(state.clone(), "file_path", "Path to source file", "\u{00a0}\u{00a0}Clip\u{00a0}\u{00a0}\u{00a0}", Callback::from(|s: Option<String>| {
        log::info!("final callback path: {}", s.unwrap());
    }))
}

#[function_component(SearchInput)]
pub fn search_input() -> Html {
    let state = use_state(|| None);
    single_text_form(state.clone(), "search_string", "Search words", "Search", Callback::from(|s: Option<String>| {
        log::info!("final callback: {}", s.unwrap());
    }))
}
