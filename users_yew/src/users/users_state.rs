use js_sys::JsString;
use service_client::{client::*, UserReply};
use web_sys::console;
use yew::{Reducible, UseReducerDispatcher};

#[derive(PartialEq, Clone)]
pub struct User {
  pub id: String,
  pub name: String,
}

#[derive(Default, PartialEq)]
pub struct UserListState {
  pub list: Vec<User>,
}

pub enum UserListAction<T: Reducible> {
  Add(String, UseReducerDispatcher<T>), // name
  Rm(String, UseReducerDispatcher<T>),  // id
  Inner(UserListInnerAction),
  Fetch(UseReducerDispatcher<T>),
}

pub enum UserListInnerAction {
  AddOne(User),
  DeleteOne(String),
  SetAll(Vec<User>),
}

impl Reducible for UserListState {
  type Action = UserListAction<UserListState>;

  fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
    match action {
      UserListAction::Add(name, dispatcher) => {
        {
          wasm_bindgen_futures::spawn_local(async move {
            let client = Client::new("http://localhost:3000").user();
            let resp = client
              .create_one(&service_client::CreateUserPayload { username: name })
              .await;

            match resp {
              Ok(user_reply) => {
                let user: User = user_reply.into();
                let action = UserListAction::Inner(UserListInnerAction::AddOne(user));
                dispatcher.dispatch(action);
              }
              Err(e) => console::log_1(&JsString::from(format!("clientError: {e:?}").as_str())),
            }
          });
        }
        self
      }
      UserListAction::Rm(id, dispatcher) => {
        {
          wasm_bindgen_futures::spawn_local(async move {
            let client = Client::new("http://localhost:3000").user();
            let resp = client.delete_one(&id).await;
            match resp {
              Ok(user_reply) => {
                let id = user_reply.id;
                let action = UserListAction::Inner(UserListInnerAction::DeleteOne(id));
                dispatcher.dispatch(action);
              }
              Err(e) => console::log_1(&JsString::from(format!("clientError: {e:?}").as_str())),
            }
          });
        }
        self
      }
      UserListAction::Fetch(dispatcher) => {
        {
          wasm_bindgen_futures::spawn_local(async move {
            let client = Client::new("http://localhost:3000").user();
            let resp = client.fetch_all().await;

            match resp {
              Ok(users) => {
                let users: Vec<User> = users.into_iter().map(|u| u.into()).collect();
                let action = UserListAction::Inner(UserListInnerAction::SetAll(users));
                dispatcher.dispatch(action);
              }
              Err(e) => console::log_1(&JsString::from(format!("clientError: {e:?}").as_str())),
            }
          });
        }
        self
      }
      UserListAction::Inner(act) => match act {
        UserListInnerAction::AddOne(u) => {
          let mut list = self.list.clone();
          list.push(u);
          Self { list }.into()
        }
        UserListInnerAction::DeleteOne(id) => {
          let list: Vec<User> = self.list.iter().filter(|u| u.id != id).cloned().collect();
          Self { list }.into()
        }
        UserListInnerAction::SetAll(list) => Self { list }.into(),
      },
    }
  }
}

impl From<UserReply> for User {
  fn from(value: UserReply) -> Self {
    User {
      id: value.id,
      name: value.username,
    }
  }
}
