mod users;

use crate::users::{UserForm, UserList, UserListStateProvider};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
  html! {
      <div class="container">
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
