use yew::prelude::*;
use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub class: String,
    pub placeholder: String,
    pub type_: String,
    pub onchange: Callback<Event>,
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let Props {
        class,
        placeholder,
        type_,
        onchange,
    } = props;

    html! {
        <input
            class={format!("text-center border border-black p-2 w-full {}", class)}
            type={type_.to_string()}
            placeholder={placeholder.to_string()}
            onchange={onchange.clone()}
        />
    }
}
