mod user_form;
mod user_list;
mod users_provider;

use crate::{user_list::UserList, users_provider::UserListStateProvider};
use user_form::UserForm;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
  html! {
      <div>
          <h1> { "Usuarios" } </h1>
          <UserListStateProvider>
            <UserForm />
            <UserList />
          </UserListStateProvider>
      </div>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
