use yew::prelude::*;

use crate::users::user_list::UserListState;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub children: Children,
}

#[function_component]
pub fn UserListStateProvider(props: &Props) -> Html {
  let users = use_reducer(UserListState::default);
  html! {
    <ContextProvider<UseReducerHandle<UserListState>> context={users}>
      {for props.children.iter()}
    </ContextProvider<UseReducerHandle<UserListState>>>
  }
}
