use yew::prelude::*;
use yew::{function_component, html, Properties};

use crate::components::button::Button;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub id: String,
    pub label: String,
    pub class: String,
    pub button_text: String,
    pub on_change: Callback<Event>,
    pub submit_disabled: bool,
    pub on_submit: Callback<MouseEvent>,
}

#[function_component(FileInput)]
pub fn file_input(props: &Props) -> Html {
    let Props {
        id,
        label,
        class,
        button_text,
        on_change,
        submit_disabled,
        on_submit,
    } = props;

    html! {
      <div class="flex flex-col">
          <div class={format!(
              "flex flex-col
              items-center
              bg-grey-light
              border-b-4 border-b-red
              text-center
              my-1 py-2 px-4
              {}", class)}
          >
              <label for={id.to_string()} class="font-bold">
                  { label.to_string() }
              </label>
              <input
                  id={id.to_string()}
                  class={format!("uppercase py-2 {}", class)}
                  type="file"
                  onchange={on_change.clone()}
              />
          </div>
          <Button
              class="my-1"
              text={button_text.to_string()}
              disabled={*submit_disabled}
              onclick={on_submit.clone()}
          />
      </div>
    }
}
