use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew::{events::Event, function_component, html, use_state, Properties};

use crate::components::{button::Button, input::Input};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_login: Callback<(String, String)>,
    pub loading: bool,
    pub error: Option<String>,
}

#[function_component(Login)]
pub fn login(props: &Props) -> Html {
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());

    let on_username_change = {
        let username = username.clone();
        Callback::from(move |ev: Event| {
            let target: EventTarget = ev.target().expect("No target found.");
            let val: String = target.unchecked_into::<HtmlInputElement>().value();
            username.set(val);
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |ev: Event| {
            let target: EventTarget = ev.target().expect("No target found.");
            let val: String = target.unchecked_into::<HtmlInputElement>().value();
            password.set(val);
        })
    };

    let on_login_click = {
        let on_login = props.on_login.clone();
        Callback::from(move |_| {
            on_login.emit((username.to_string(), password.to_string()));
        })
    };

    html! {
        <div class="flex flex-col m-auto w-80 before:block before:bg-red before:w-12 before:h-2">
            <h1 class="my-8 text-3xl">
                { "CAPITALISM / LOGIN" }
            </h1>

            <Input
                class="my-2"
                type_="text"
                placeholder="USERNAME"
                onchange={on_username_change}
            />
            <Input
                class="my-2"
                type_="password"
                placeholder="PASSWORD"
                onchange={on_password_change}
            />
            <Button
                class="mt-4"
                text="LOGIN"
                onclick={on_login_click}
            />
            if let Some(err) = &props.error {
                <p class="my-2 text-center">
                    { err }
                </p>
            }

            if props.loading {
                <p class="my-2 text-center">
                    { "Loading..." }
                </p>
            }
        </div>
    }
}
