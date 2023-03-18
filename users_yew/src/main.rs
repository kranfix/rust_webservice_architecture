mod user_list;
use crate::user_list::{User, UserList};
use web_sys::HtmlInputElement;
use yew::{html::IntoPropValue, prelude::*};

#[function_component]
fn App() -> Html {
  let input_node_ref = use_node_ref();
  let name = use_state(String::default);
  let oninput = {
    let name = name.clone();
    let input_node_ref = input_node_ref.clone();
    move |e: InputEvent| {
      let input = input_node_ref.cast::<HtmlInputElement>();

      if let Some(input) = input {
        name.set(input.value().trim().into());
      }
    }
  };

  let mut users = Vec::new();
  //users.push(User {
  //  id: "abc".to_string(),
  //  name: "Frank".to_string(),
  //});

  html! {
      <div>
          <h1> { "Usuarios" } </h1>
          <input ref={input_node_ref} type="text" {oninput}/>
          <button disabled={name.is_empty()}>{ "Add user" }</button>
          <UserList {users}/>
      </div>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
