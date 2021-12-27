use yew::prelude::*;
use yew::{html, Context, Properties};

pub struct Button;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub class: String,
    pub text: String,
    pub onclick: Callback<Event>,
}

impl Component for Button {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            class,
            text,
            onclick,
        } = ctx.props();

        html! {
            <button onchange={onclick.clone()} class={format!(
                "bg-grey-light hover:bg-grey transition border-b border-b-4 border-b-red p-2 font-bold {}",
                class
            )}>
                { text }
            </button>
        }
    }
}
