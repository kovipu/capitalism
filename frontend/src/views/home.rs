use wasm_bindgen::JsCast;
use web_sys::{EventTarget, File, FileList, HtmlInputElement};
use yew::prelude::*;
use yew::{function_component, use_state, Event, Properties};
use yew_router::prelude::Redirect;

use crate::components::fileinput::FileInput;
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub auth: String,
}

#[function_component(Home)]
pub fn home(props: &Props) -> Html {
    // if auth is empty, redirect.
    if props.auth.is_empty() {
        return html! {
            <Redirect<Route> to={Route::Login} />
        };
    }

    let input_file: UseStateHandle<Option<File>> = use_state(|| None);

    let on_file_change = {
        let input_file = input_file.clone();

        Callback::from(move |ev: Event| {
            let target: EventTarget = ev.target().expect("No target found.");
            let files: FileList = target
                .unchecked_into::<HtmlInputElement>()
                .files()
                .expect("No FileList found.");

            if let Some(file) = files.item(0) {
                log::info!("File: {:?}", file);
                input_file.set(Some(file));
            }
        })
    };

    let on_file_submit = Callback::from(move |_| {
        log::info!("on_file_submit");
    });

    html! {
        <div class="flex flex-col max-w-2xl mx-auto my-10 before:block before:bg-red before:w-12 before:h-2">
            <h2 class="mt-12 mb-8 text-xl font-bold">
                { "TRANSACTIONS" }
            </h2>

            <div class="p-2 my-4 border border-black">
                { "// TRANSACTIONS GO HERE" }
            </div>
            <FileInput
                id="statement-input"
                label="UPLOAD A STATEMENT"
                class=""
                button_text="SEND"
                on_change={on_file_change.clone()}
                submit_disabled={input_file.is_none()}
                on_submit={on_file_submit.clone()}
             />
        </div>
    }
}
