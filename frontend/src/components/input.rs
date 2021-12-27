use yew::prelude::*;
use yew::{html, Context, Properties};

pub struct Input;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub class: String,
    pub placeholder: String,
    pub type_: String,
    pub onchange: Callback<Event>,
}

impl Component for Input {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            class,
            placeholder,
            type_,
            onchange,
        } = ctx.props();

        html! {
            <input
                class={format!("text-center border border-black p-2 w-full {}", class)}
                type={type_.to_string()}
                placeholder={placeholder.to_string()}
                onchange={onchange.clone()}
            />
        }
    }
}
