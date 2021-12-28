use yew::prelude::*;
use yew::{function_component, Properties};
use yew_router::prelude::Redirect;

use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub auth: String,
}

#[function_component(Home)]
pub fn home(props: &Props) -> Html {
    if props.auth.is_empty() {
        return html! {
            <Redirect<Route> to={Route::Login} />
        };
    }
    // if auth is empty, redirect.
    html! {
        <div>
            <p>{ "Hello world!" }</p>
        </div>
    }
}
