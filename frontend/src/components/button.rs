use yew::prelude::*;
use yew::{events::MouseEvent, function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub class: String,
    pub text: String,
    pub disabled: bool,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let Props {
        onclick,
        class,
        disabled,
        text,
    } = props;

    html! {
        <button onclick={onclick.clone()} disabled={*disabled} class={format!(
            "bg-grey-light
            hover:bg-grey
            disabled:opacity-50 disabled:bg-grey-light
            transition ease-in-out
            border-b-4 border-b-red
            py-2 px-4 font-bold
            {}",
            class
        )}>
            { text }
        </button>
    }
}
