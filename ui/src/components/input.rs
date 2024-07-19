use yew::prelude::*;

#[function_component(FileInput)]
pub fn file_input() -> Html {
    html! {
        <form method="POST" enctype="multipart/form-data" action="/clip_from_file">
            <fieldset role="group">
                <input type="text"
                    name="file_path"
                    placeholder="Path to source file"
                    aria-label="Path to source file"
                    required=true
                />
                <button type="submit">{ "\u{00a0}\u{00a0}Clip\u{00a0}\u{00a0}\u{00a0}" }</button>
            </fieldset>
        </form>
    }
}

#[function_component(SearchInput)]
pub fn search_input() -> Html {
    html! {
        <form method="POST" enctype="multipart/form-data" action="/search">
            <fieldset role="group">
                <input type="text"
                    name="search_string"
                    placeholder="Search words"
                    aria-label="Search words"
                    required=true
                />
                <button type="submit">{ "Search" }</button>
            </fieldset>
        </form>
    }
}