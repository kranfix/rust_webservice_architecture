use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
  pub on_add: Callback<String>,
}

#[function_component]
pub fn UserForm(props: &Props) -> Html {
  let input_node_ref = use_node_ref();
  let name = use_state(String::default);
  let oninput = {
    let name = name.clone();
    let input_node_ref = input_node_ref.clone();
    move |_: InputEvent| {
      let input = input_node_ref.cast::<HtmlInputElement>();

      if let Some(input) = input {
        name.set(input.value().trim().into());
      }
    }
  };

  let onclick = {
    let on_add = props.on_add.clone();
    let name = (*name).clone();
    move |_| {
      on_add.emit(name.clone());
    }
  };

  html! {
    <>
      <input ref={input_node_ref} type="text" {oninput}/>
      <button disabled={name.is_empty()} {onclick}>{ "Add user" }</button>
    </>
  }
}
