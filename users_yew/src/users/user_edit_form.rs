use js_sys::JsString;
use web_sys::console;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::users::UserController;

use super::User;

#[derive(PartialEq, Properties)]
pub struct UserEditFormProps {
  pub user: User,
  pub on_edited: Callback<()>,
}

#[function_component]
pub fn UserEditForm(props: &UserEditFormProps) -> Html {
  let edit_node_ref = use_node_ref();
  let name = use_state(|| props.user.name.clone());
  let oninput = {
    let name = name.clone();
    let edit_node_ref = edit_node_ref.clone();
    move |_: InputEvent| {
      let input = edit_node_ref.cast::<HtmlInputElement>();
      if let Some(input) = input {
        name.set(input.value().trim().into());
      }
    }
  };

  let name = { (*name).clone() };

  let close_dialog = {
    let on_edit = props.on_edited.clone();
    move |_: yew::MouseEvent| on_edit.emit(())
  };
  let on_edited = props.on_edited.clone();
  html! {
    <>
      <form>
        <div class="mb-3">
          <input ref={edit_node_ref} type="text" {oninput} value={name.clone()} class="form-control"/>
        </div>
      </form>

      <EditUserButton user={props.user.clone()} new_name={name} on_edited={on_edited}/>
      <button onclick={close_dialog.clone()} type="button" class="btn btn-outline-secondary" style="margin-left:0.5rem">{ "X" }</button>
    </>
  }
}

// TODO(Frank): enhance API
#[derive(PartialEq, Properties)]
pub struct EditUserButtonProps {
  pub user: User,
  pub new_name: String,
  pub on_edited: Callback<()>,
}

#[function_component]
fn EditUserButton(props: &EditUserButtonProps) -> Html {
  let users = use_context::<UserController>().unwrap();
  let onclick = {
    let id = props.user.id.clone();
    let new_name = props.new_name.clone();
    let on_edited = props.on_edited.clone();
    move |_| {
      users.clone().update(id.clone(), new_name.clone());
      console::log_1(&JsString::from(format!("{new_name:?}").as_str()));
      on_edited.clone().emit(());
    }
  };

  html!(
    <button disabled={props.user.name.is_empty()} {onclick}  type="button" class="btn btn-primary">{ "Edit user" }</button>
  )
}
