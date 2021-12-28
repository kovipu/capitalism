use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;
use crate::views::{home::Home, login::Login};

#[function_component(App)]
pub fn app() -> Html {
    let history = use_history().unwrap();
    let auth = use_state(|| "".to_string());

    let login_callback = {
        let auth = auth.clone();
        Callback::from(move |new_auth| {
            auth.set(new_auth);
            history.push(Route::Home);
        })
    };

    let switch = move |route: &Route| match route {
        Route::Home => html! {
            <Home auth={auth.to_string()} />
        },
        Route::Login => html! {
            <Login callback={login_callback.clone()} />
        },
    };

    html! {
        <div class="flex flex-col min-h-screen">
            <Switch<Route> render={Switch::render(switch)} />
        </div>
    }
}
