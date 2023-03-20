use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
  pub users: Vec<User>,
  pub on_delete: Callback<String>,
}

#[derive(PartialEq, Clone)]
pub struct User {
  pub id: String,
  pub name: String,
}

#[function_component]
pub fn UserList(props: &Props) -> Html {
  let users = props.users.clone();
  let on_delete = props.on_delete.clone();
  html! {
      <div>
        if users.is_empty(){
          <p>{ "Please add users" }</p>
        } else {
          <ul>
            {
              users.into_iter().map(|u| {
                let on_delete = on_delete.clone();
                let id = u.id.clone();
                let onclick = {
                  move |_| {
                    on_delete.clone().emit(id.clone());
                  }
                };
                html!(
                  <li>
                    {u.name.clone()}
                    <button {onclick}>{ "X" }</button>
                  </li>
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

#[derive(Default)]
pub struct UserListState {
  pub list: Vec<User>,
}

pub enum UserListAction {
  Add(String), // name
  Rm(String),  // id
}

impl Reducible for UserListState {
  type Action = UserListAction;

  fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
    match action {
      UserListAction::Add(name) => {
        let u = User {
          id: self.list.len().to_string(),
          name,
        };

        let mut list = self.list.clone();
        list.push(u);
        Self { list }.into()
      }
      UserListAction::Rm(id) => {
        let list: Vec<User> = self.list.iter().filter(|u| u.id != id).cloned().collect();
        Self { list }.into()
      }
    }
  }
}
