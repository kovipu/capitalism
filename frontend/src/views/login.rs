use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew::{events::Event, html, Component, Html};

use crate::components::{button::Button, input::Input};

pub struct Login {
    username: String,
    password: String,
}

pub enum Msg {
    UpdateUsername(String),
    UpdatePassword(String),
    Login,
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            username: String::new(),
            password: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateUsername(username) => self.username = username,
            Msg::UpdatePassword(password) => self.password = password,
            Msg::Login => {
                println!("{}", self.username);
                println!("{}", self.password);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_username_change = link.callback(|ev: Event| {
            let target: EventTarget = ev
                .target()
                .expect("Event should have a target when dispatched.");
            Msg::UpdateUsername(target.unchecked_into::<HtmlInputElement>().value())
        });

        let on_password_change = link.callback(|ev: Event| {
            let target: EventTarget = ev
                .target()
                .expect("Event should have a target when dispatched.");
            Msg::UpdatePassword(target.unchecked_into::<HtmlInputElement>().value())
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
                    onclick={link.callback(|_| Msg::Login)}
                />
            </div>
        }
    }
}
