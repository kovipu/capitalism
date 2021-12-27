use yew::prelude::*;

mod components;
mod views;

use views::login::Login;

struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex flex-col h-screen">
                <Login/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
