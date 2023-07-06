use leptos::*;

use crate::users::create_user_controller;

#[component]
pub fn UserListStateProvider(cx: Scope, children: Children) -> impl IntoView {
  let controller = create_user_controller(cx);
  provide_context(cx, controller);
  view! {
    cx,
    <>
      {children(cx).into_view(cx)}
    </>
  }
}
