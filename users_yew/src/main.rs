mod users;

use crate::users::{ClientStateProvider, UserForm, UserList, UserListStateProvider};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
  html! {
      <ClientStateProvider>
        <div class="container">
            <h1> { "Usuarios" } </h1>
            <UserListStateProvider>
              <UserForm />
              <UserList />
            </UserListStateProvider>
        </div>
      </ClientStateProvider>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
