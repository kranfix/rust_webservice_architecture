//use crate::users::UserListState;
use leptos::*;
use service_client::client::*;

#[component]
pub fn ClientStateProvider(cx: Scope, children: Children) -> impl IntoView {
  let client = create_memo(cx, |_| Client::new("http://localhost:3000"));
  provide_context(cx, client);
  view! {
    cx,
    <>
      {children(cx).into_view(cx)}
    </>
  }
}
