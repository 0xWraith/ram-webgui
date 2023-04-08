#![allow(non_camel_case_types)]

use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub on_change: Callback<String>,
  #[prop_or_default]
  pub default_value: AttrValue,
}

#[function_component(InputComponent)]
pub fn input_component(props: &Props) -> Html {
  let on_change = props.on_change.clone();
  let value = use_state(|| props.default_value.to_string());
  let value_cloned = value.clone();

  let handle_change = move |event: InputEvent| {
    if let Some(input) = event.target_dyn_into::<HtmlTextAreaElement>() {
      value_cloned.set(input.value());
      on_change.emit(input.value());
    } else {
      log::error!("Failed to cast event target to HtmlTextAreaElement");
    }
  };

  html! {
    <>
      <textarea
        class="user-input"
        placeholder="Enter input"
        oninput={handle_change}
        value={value.to_string()}
      />
    </>
  }
}
