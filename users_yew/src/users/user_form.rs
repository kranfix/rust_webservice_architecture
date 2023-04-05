use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::users::{UserListAction, UserListState};

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
      <form>
        <div class="mb-3">
          <input ref={input_node_ref} type="text" {oninput} value={name.clone()} class="form-control" aria-describedby="emailHelp"/>
          <div id="emailHelp" class="form-text">{"Name"}</div>
        </div>
      </form>

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

    let dispatch = users.dispatcher();
    move |_| {
      let action = UserListAction::Add(name.clone(), dispatch.clone());
      users.clone().dispatch(action);
      on_added.clone().emit(())
    }
  };

  html!(
    <button disabled={props.name.is_empty()} {onclick}  type="button" class="btn btn-primary">{ "Add user" }</button>
  )
}
