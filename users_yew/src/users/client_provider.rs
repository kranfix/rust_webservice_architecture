use yew::prelude::*;

//use crate::users::UserListState;
use service_client::client::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub children: Children,
}

#[function_component]
pub fn ClientStateProvider(props: &Props) -> Html {
  let client = Client::new("http://localhost:3000");
  html! {
    <ContextProvider<Client> context={client}>
      {for props.children.iter()}
    </ContextProvider<Client>>
  }
}
