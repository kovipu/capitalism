use yew::prelude::*;

mod api;
mod components;

mod views;

use views::login::Login;

#[function_component(App)]
fn app() -> Html {
    let auth = use_state(|| "".to_string());

    let login_callback = {
        let auth = auth.clone();
        Callback::from(move |new_auth| {
            auth.set(new_auth);
        })
    };

    html! {
        <div class="flex flex-col h-screen">
            <Login callback={login_callback} />
        </div>
    }
}

fn main() {
    // set up wasm-logger, to convert Rust's log into console-calls.
    wasm_logger::init(wasm_logger::Config::default());

    // Start the Yew app.
    yew::start_app::<App>();
}
