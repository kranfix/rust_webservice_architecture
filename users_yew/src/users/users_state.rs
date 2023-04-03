use std::rc::Rc;

use js_sys::JsString;
use reqwest::header::{ACCESS_CONTROL_ALLOW_HEADERS, CONTENT_TYPE};
use service_client::{Reply, UserReply};
use web_sys::console;
use yew::Reducible;

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
  Add(String, Rc<dyn Fn(<T as Reducible>::Action)>), // name
  Rm(String, Rc<dyn Fn(<T as Reducible>::Action)>),  // id
  Inner(UserListInnerAction),
  Fetch(Rc<dyn Fn(<T as Reducible>::Action)>),
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
      UserListAction::Add(name, dispatch) => {
        {
          /*
           {
             data: {
               id: String,
               username: String
             }
           }
          */
          let name = name.clone();
          wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest::Client::new();
            let reqwest = client
              .post(format!("{}/users", "http://localhost:3000"))
              .header("Accept", "application/json")
              .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
              .header(CONTENT_TYPE, "application/json")
              //.fetch_mode_no_cors()
              .body(format!(r#"{{"username":"{name}"}}"#));
            let resp = reqwest
              .send()
              .await
              .expect("HTTP ERROR")
              .json::<Reply<UserReply>>()
              .await
              .expect("QueryResult parse error");

            match resp {
              Reply::Data(user_reply) => {
                let user: User = user_reply.into();
                let action = UserListAction::Inner(UserListInnerAction::AddOne(user));
                dispatch(action);
              }
              Reply::Err(e) => console::log_1(&JsString::from(e.as_str())),
            }
          });
        }
        self
      }
      UserListAction::Rm(id, dispatch) => {
        {
          let id = id.clone();
          wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest::Client::new();
            let reqwest = client
              .delete(format!("{}/users/{}", "http://localhost:3000", id))
              .header("Accept", "application/json")
              .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
              .header(CONTENT_TYPE, "application/json");
            let resp = reqwest
              .send()
              .await
              .expect("HTTP ERROR")
              .json::<Reply<UserReply>>()
              .await
              .expect("QueryResult parse error");

            match resp {
              Reply::Data(user_reply) => {
                let id = user_reply.id;
                let action = UserListAction::Inner(UserListInnerAction::DeleteOne(id));
                dispatch(action);
              }
              Reply::Err(e) => console::log_1(&JsString::from(e.as_str())),
            }
          });
        }
        self
      }
      UserListAction::Fetch(dispatch) => {
        {
          wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest::Client::new();
            let reqwest = client
              .get(format!("{}/users", "http://localhost:3000"))
              .header("Accept", "application/json")
              .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
              .header(CONTENT_TYPE, "application/json");
            let resp = reqwest
              .send()
              .await
              .expect("HTTP ERROR")
              .json::<Reply<Vec<UserReply>>>()
              .await
              .expect("QueryResult parse error");

            match resp {
              Reply::Data(users) => {
                let users: Vec<User> = users.into_iter().map(|u| u.into()).collect();
                let action = UserListAction::Inner(UserListInnerAction::SetAll(users));
                dispatch(action);
              }
              Reply::Err(e) => console::log_1(&JsString::from(e.as_str())),
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
