use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct User {
  pub id: String,
  pub name: String,
}

#[function_component]
pub fn UserList() -> Html {
  //let users = props.users.clone();
  let users = use_context::<UseReducerHandle<UserListState>>().unwrap();
  //let on_delete = props.on_delete.clone();
  let on_delete = {
    let users = users.clone();
    move |id: String| {
      let action = UserListAction::Rm(id.clone());
      users.dispatch(action);
    }
  };
  html! {
      <div class="mt-4">
        if users.list.is_empty(){
          <div id="emailHelp" class="form-text">{ "Please add users" }</div>
        } else {
          <ul class="list-group">
            {
              users.list.iter().map(|u| {
                let on_delete = on_delete.clone();
                let id = u.id.clone();
                let onclick = {
                  move |_| {
                    on_delete.clone()(id.clone());
                  }
                };
                html!(
                  <div class="container text-start">
                    <div class="row align-items-center">
                      <div class="col mt-1">
                        <li class="list-group-item">
                          {u.name.clone()}
                        </li>
                      </div>
                      <div class="col mb-1">
                        <button {onclick} type="button" class="btn btn-outline-danger">{ "X" }</button>
                      </div>
                    </div>
                  </div>
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

#[derive(Default, PartialEq)]
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
