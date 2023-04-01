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
  Rm(String),                                        // id
  Inner(UserListInnerAction),
}

pub enum UserListInnerAction {
  Set(User),
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
              Reply::data(user_reply) => {
                let user: User = user_reply.into();
                let action = UserListAction::Inner(UserListInnerAction::Set(user));
                dispatch(action);
              }
              Reply::err(e) => console::log_1(&JsString::from(e.as_str())),
            }
          });
        }
        self
      }
      UserListAction::Rm(id) => {
        let list: Vec<User> = self.list.iter().filter(|u| u.id != id).cloned().collect();
        Self { list }.into()
      }
      UserListAction::Inner(act) => match act {
        UserListInnerAction::Set(u) => {
          let mut list = self.list.clone();
          list.push(u);
          Self { list }.into()
        }
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
