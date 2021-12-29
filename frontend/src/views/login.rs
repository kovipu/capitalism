use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew::{events::Event, function_component, html, use_state, Properties};

use crate::api;
use crate::components::{button::Button, input::Input};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub callback: Callback<String>,
}

#[function_component(Login)]
pub fn login(props: &Props) -> Html {
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());

    let error: UseStateHandle<Option<String>> = use_state(|| None);

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

    let button_disabled = username.is_empty() || password.is_empty();

    let on_login_click = {
        let callback = props.callback.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let username = username.clone();
            let password = password.clone();
            let error = error.clone();
            let callback = callback.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let auth: String = api::build_auth_header(&username, &password);
                let result = api::login(&auth).await;

                match result {
                    Err(_) => {
                        error.set(Some("Login request failed.".to_string()));
                    }
                    Ok(response) => {
                        if response.status() == 200 {
                            callback.emit(auth);
                        } else {
                            let message = response.text().await.unwrap();
                            error.set(Some(message));
                        }
                    }
                }
            });
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
                disabled={button_disabled}
                onclick={on_login_click}
            />
            if let Some(err) = &*error {
                <p class="my-2 text-center">
                    { err }
                </p>
            }
        </div>
    }
}
