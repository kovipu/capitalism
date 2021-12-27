use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew::{events::Event, function_component, html, use_state};

use crate::components::{button::Button, input::Input};

#[function_component(Login)]
pub fn login() -> Html {
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

    let on_login_click = Callback::from(move |_| {
        // request login.
    });

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
        </div>
    }
}
