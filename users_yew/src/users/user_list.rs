use crate::users::{User, UserController, UserEditForm};
use web_sys::*;
use yew::prelude::*;

#[function_component]
pub fn UserList() -> Html {
  let users = use_context::<UserController>().unwrap();
  let user_to_edit: UseStateHandle<Option<User>> = use_state(|| None);

  let on_delete = {
    let users = users.clone();
    move |id: String| users.rm(id)
  };
  let dialog_ref = use_node_ref();
  let open_update_user_dialog = {
    let dialog_ref = dialog_ref.clone();
    let user_to_edit = user_to_edit.clone();
    move |user: User| {
      let modal = dialog_ref.cast::<HtmlDialogElement>();

      if let Some(modal) = modal {
        match modal.show_modal() {
          Ok(_) => user_to_edit.set(Some(user)),
          Err(_) => user_to_edit.set(None),
        }
      }
    }
  };

  let close_user_edit_dialog = {
    let user_to_edit = user_to_edit.clone();
    let dialog_ref = dialog_ref.clone();
    move |_| {
      user_to_edit.set(None);
      let modal = dialog_ref.cast::<HtmlDialogElement>();

      if let Some(modal) = modal {
        modal.close();
      }
    }
  };

  {
    let users = users.clone();
    use_effect(move || {
      users.fetch_all();
      || {}
    });
  }

  html! {
      <div class="mt-4">
        if users.list.is_empty(){
          <div id="emailHelp" class="form-text">{ "Please add users" }</div>
        } else {
          <ul class="list-group">
            {
              users.list.iter().map(|u| {
                let on_delete = on_delete.clone();
                let open_update_user_dialog = open_update_user_dialog.clone();
                let id = u.id.clone();
                let onclickx = {
                  let id = id.clone();
                  move |_| {
                    on_delete.clone()(id.clone());
                  }
                };
                let open_update_dialog = {
                  let user = u.clone();
                  move |_| {
                    open_update_user_dialog.clone()(user.clone());
                  }
                };
                html!(
                  <div class="container text-start">
                    <div class="row align-items-center">
                      <div class="col mt-1">
                        <li class="list-group-item">
                          {u.name.clone()}
                        </li>
                      </div>
                      <div class="col mb-1">
                        <button onclick={open_update_dialog} type="button" class="btn btn-outline-danger">{ "U" }</button>
                        <button onclick={onclickx} type="button" class="btn btn-outline-danger" style="margin-left:0.5rem">{ "X" }</button>
                      </div>
                    </div>
                  </div>
                )
              }).collect::<Html>()
            }
          </ul>
        }
        <dialog ref={dialog_ref}>
          if let Some(user) = user_to_edit.as_ref().cloned() {
            <UserEditCard {user} on_edited={close_user_edit_dialog}/>
          }
          //<button onclick={close_user_edit_dialog} type="button" class="btn btn-outline-secondary" style="margin-left:0.5rem">{ "X" }</button>
        </dialog>
      </div>
  }
}

#[derive(Properties, PartialEq)]
pub struct Props {
  pub user: User,
  on_edited: Callback<()>,
}

#[function_component]
fn UserEditCard(props: &Props) -> Html {
  let user = props.user.clone();
  let close_dialog = props.on_edited.clone();
  html! {
    <div>
      <h2>{"Edit user"}</h2>
      <span>{user.name.clone()}</span>
      <UserEditForm user={user.clone()} on_edited={close_dialog}/>
    </div>
  }
}
