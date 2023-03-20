mod user_form;
mod user_list;

use crate::user_list::{UserList, UserListAction, UserListState};
use user_form::UserForm;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
  let users = use_reducer(UserListState::default);
  let on_add = {
    let users = users.clone();
    Callback::from(move |name: String| {
      let action = UserListAction::Add(name.clone());
      users.dispatch(action);
    })
  };

  let on_delete = {
    let users = users.clone();
    Callback::from(move |id: String| {
      let action = UserListAction::Rm(id.clone());
      users.dispatch(action);
    })
  };

  html! {
      <div>
          <h1> { "Usuarios" } </h1>
          <UserForm {on_add}/>
          <UserList users={users.list.clone()} {on_delete} />
      </div>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
