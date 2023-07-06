use std::rc::Rc;

use js_sys::JsString;
use leptos::{html::Input, *};
use web_sys::console;
//use web_sys::HtmlInputElement;

use crate::users::UserController;

use super::User;

#[component]
pub fn UserEditForm<E>(cx: Scope, user: User, on_edited: E) -> impl IntoView
where
  E: Fn() + 'static,
{
  let edit_node_ref = create_node_ref::<Input>(cx);
  let (name, set_name) = create_signal(cx, user.name.clone());
  //let name = create_signal(user.name.clone()); //se uso un use_state
  let oninput = {
    let name = name.clone();
    let edit_node_ref = edit_node_ref.clone();
    move || {
      let input = edit_node_ref.get();
      if let Some(input) = input {
        set_name(input.value().trim().into());
      }
    }
  };

  let on_edited_rc = Rc::new(on_edited);
  let close_dialog = {
    let on_edited = on_edited_rc.clone();
    move |_| on_edited()
  };

  let edit = move || on_edited_rc();

  view! (cx,
    <>
      <form>
        <div class="mb-3">
          <input ref={edit_node_ref} type="text" {oninput} value=name.clone() class="form-control"/>
        </div>
      </form>

      <EditUserButton user={user.clone()} new_name={name()} on_edited={edit}/>
      <button on:click=close_dialog type="button" class="btn btn-outline-secondary" style="margin-left:0.5rem">{ "X" }</button>
    </>
  )
}

#[component]
fn EditUserButton<E>(cx: Scope, user: User, new_name: String, on_edited: E) -> impl IntoView
where
  E: Fn() + 'static,
{
  let users = use_context::<UserController>(cx).unwrap();
  let onclick = {
    let id = user.id.clone();
    let new_name = new_name.clone();
    let on_edited = on_edited;
    move |_| {
      users.clone().update(id.clone(), new_name.clone());
      console::log_1(&JsString::from(format!("{new_name:?}").as_str()));
      on_edited();
    }
  };

  view!(cx,
    <button disabled=user.name.is_empty() on:click=onclick type="button" class="btn btn-primary">{ "Edit user" }</button>
  )
}
