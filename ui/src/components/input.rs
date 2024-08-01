use leptos::*;

#[component]
fn SingleTextForm(
    name: &'static str,
    placeholder: &'static str,
    button_text: &'static str,
    #[prop(into)] callback: Callback<String>,
) -> impl IntoView {
    let input_element: NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let Some(value) = input_element.get() else {
            return;
        };
        callback.call(value.value());
    };
    view! {
        <form on:submit=on_submit> // on_submit defined below
            <fieldset role="group">
                <input type="text"
                    name=name
                    placeholder=placeholder
                    arial-label=placeholder
                    required=true

                    node_ref=input_element
                />
                <button type="submit">{button_text}</button>
            </fieldset>
        </form>
    }
}

#[component]
pub fn ExactPathInput(#[prop(into)] callback: Callback<String>) -> impl IntoView {
    view! {
        <SingleTextForm
            name="file_path"
            placeholder="Path to source file"
            button_text="\u{00a0}\u{00a0}Clip\u{00a0}\u{00a0}\u{00a0}"
            callback=callback
        />
    }
}

#[component]
pub fn SearchInput(#[prop(into)] callback: Callback<String>) -> impl IntoView {
    view! {
        <SingleTextForm
            name="search_string"
            placeholder="Search words"
            button_text="Search"
            callback=callback
        />
    }
}
