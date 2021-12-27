use yew::function_component;
use yew::prelude::*;

mod api;
mod components;
mod views;

use views::login::Login;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="flex flex-col h-screen">
            <Login/>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
