use yew::prelude::*;
use yew_router::prelude::*;

mod api;
mod app;
mod components;
mod routes;
mod views;

use app::App;

#[function_component(Main)]
fn mainwrapper() -> Html {
    html! {
        <BrowserRouter>
            <App />
        </BrowserRouter>
    }
}

fn main() {
    // set up wasm-logger, to convert Rust's log into console-calls.
    wasm_logger::init(wasm_logger::Config::default());

    // Start the Yew app.
    yew::start_app::<Main>();
}
