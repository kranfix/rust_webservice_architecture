use service_client::client::Client;
use yew::prelude::*;

use crate::users::{UserController, UserListState};

#[derive(Properties, PartialEq)]
pub struct Props {
  pub children: Children,
}

#[function_component]
pub fn UserListStateProvider(props: &Props) -> Html {
  let users = use_reducer(UserListState::default);
  let client = use_context::<Client>().unwrap().user();
  let controller = UserController::new(users, client);
  html! {
    <ContextProvider<UserController> context={controller}>
      {for props.children.iter()}
    </ContextProvider<UserController>>
  }
}
