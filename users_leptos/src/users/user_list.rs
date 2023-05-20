use crate::users::{User, UserController, UserEditForm};
use leptos::{html::Dialog, *};

#[component]
pub fn UserList(cx: Scope) -> impl IntoView {
  let users = use_context::<UserController>(cx).unwrap();
  let (user_to_edit, set_user_to_edit) = create_signal(cx, None);

  let on_delete = {
    let users = users.clone();
    move |id: String| users.rm(id)
  };
  let dialog_ref = create_node_ref::<Dialog>(cx);
  let open_update_user_dialog = {
    let dialog_ref = dialog_ref.clone();
    let user_to_edit = user_to_edit.clone();
    move |user: User| {
      if let Some(modal) = dialog_ref.get() {
        match modal.show_modal() {
          Ok(_) => set_user_to_edit(Some(user)),
          Err(_) => set_user_to_edit(None),
        }
      }
    }
  };

  let close_user_edit_dialog = move || {
    set_user_to_edit(None);
    if let Some(modal) = dialog_ref.get() {
      modal.close();
    }
  };

  create_effect(cx, move |_| {
    users.fetch_all();
  });

  view! {
    cx,
      <div class="mt-4">
        {if users.list.is_empty() {
          view! {
            cx,
            <>
              <div id="emailHelp" class="form-text">{ "Please add users" }</div>
            </>
          }
        } else {
          view! {
            cx,
            <>
              <ul class="list-group">
                <For
                  each=users.users
                  key=|user: &User| user.id.clone()
                  view=move |cx, user: User| {
                    view! {
                      cx,
                      <UserListTile user={user} on_delete={on_delete} open_update_user_dialog={open_update_user_dialog}/>
                    }
                  }
                />
              </ul>
            </>
          }
        }}
        <dialog ref={dialog_ref}>
          {if let Some(user) = user_to_edit() {
            view! {
              cx,
              <>
                <div>"Implementar UserEditCard"</div>
                <UserEditCard user={user} on_edited={close_user_edit_dialog}/>
              </>
            }
          } else{
            view! {
            cx,
            <></>
            }
          }}
          //<button onclick={close_user_edit_dialog} type="button" class="btn btn-outline-secondary" style="margin-left:0.5rem">{ "X" }</button>
        </dialog>
      </div>
  }
}

#[component]
fn UserEditCard<E>(cx: Scope, user: User, on_edited: E) -> impl IntoView
where
  E: Fn() + 'static,
{
  let user = user.clone();
  //let close_dialog = on_edited.clone();
  view! {
    cx,
    <div>
      <h2>{"Edit user"}</h2>
      <span>{user.name.clone()}</span>
      //<UserEditForm user={user} on_edited={on_edited} />
    </div>
  }
}

#[component]
fn UserListTile<D, U>(
  cx: Scope,
  user: User,
  on_delete: D,
  open_update_user_dialog: U,
) -> impl IntoView
where
  D: Fn(String) + 'static,
  U: Fn(User) + 'static,
{
  let id = user.id.clone();
  let onclickx = {
    let id = id.clone();
    move |_| {
      on_delete(id.clone());
    }
  };
  let open_update_dialog = {
    let user = user.clone();
    move |_| {
      open_update_user_dialog(user.clone());
    }
  };

  view! {
    cx,
    <div class="container text-start">
      <div class="row align-items-center">
        <div class="col mt-1">
          <li class="list-group-item">
            {user.name.clone()}
          </li>
        </div>
        <div class="col mb-1">
          <button on:click=open_update_dialog type="button" class="btn btn-outline-danger">{ "U" }</button>
          <button on:click=onclickx type="button" class="btn btn-outline-danger" style="margin-left:0.5rem">{ "X" }</button>
        </div>
      </div>
    </div>
  }
}
