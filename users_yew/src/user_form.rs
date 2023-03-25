use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::user_list::{UserListAction, UserListState};

// #[derive(PartialEq, Properties)]
// pub struct Props {
//   pub on_add: Callback<String>,
// }

#[function_component]
pub fn UserForm() -> Html {
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

  let clear_text = {
    let name = name.clone();
    Callback::from(move |_: ()| {
      name.set("".to_string());
    })
  };

  let name = { (*name).clone() };
  html! {
    <>
      <input ref={input_node_ref} type="text" {oninput} value={name.clone()}/>
      <AddUserButton name={name} on_added={clear_text}/>
    </>
  }
}
#[derive(PartialEq, Properties)]
pub struct AddUserButtonProps {
  name: String,
  on_added: Callback<()>,
}

#[function_component]
fn AddUserButton(props: &AddUserButtonProps) -> Html {
  let users = use_context::<UseReducerHandle<UserListState>>().unwrap();

  let onclick = {
    let name = props.name.clone();
    let on_added = props.on_added.clone();
    move |_| {
      let action = UserListAction::Add(name.clone());
      users.clone().dispatch(action);
      on_added.clone().emit(())
    }
  };

  html!(
    <button disabled={props.name.is_empty()} {onclick}>{ "Add user" }</button>
  )
}
