use yew::prelude::*;
use yew::{events::MouseEvent, function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub class: String,
    pub text: String,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let Props {
        onclick,
        class,
        text,
    } = props;

    html! {
        <button onclick={onclick.clone()} class={format!(
            "bg-grey-light hover:bg-grey transition border-b-4 border-b-red p-2 font-bold {}",
            class
        )}>
            { text }
        </button>
    }
}
