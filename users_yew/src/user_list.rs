use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
  pub users: Vec<User>,
}

#[derive(PartialEq, Clone)]
pub struct User {
  pub id: String,
  pub name: String,
}

#[function_component]
pub fn UserList(props: &Props) -> Html {
  let users = props.users.clone();
  html! {
      <div>
        if users.is_empty(){
          <p>{ "Please add users" }</p>
        } else {
          <ul>
            {
              users.into_iter().map(|u| {
                html!(
                  <li>{u.name.clone()}</li>
                )
              }).collect::<Html>()
            }
          </ul>
        }
      </div>
  }
}

#[function_component]
fn UserCard() -> Html {
  html! {}
}
