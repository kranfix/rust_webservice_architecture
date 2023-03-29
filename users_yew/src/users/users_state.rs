use std::fmt::format;

use js_sys::JsString;
use reqwest::header::{ACCESS_CONTROL_ALLOW_HEADERS, CONTENT_TYPE};
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

pub enum UserListAction {
  Add(String), // name
  Rm(String),  // id
}

impl Reducible for UserListState {
  type Action = UserListAction;

  fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
    match action {
      UserListAction::Add(name) => {
        {
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
              //.json::<Vec<QueryResult<T>>>()
              .text()
              .await
              .expect("QueryResult parse error");
            console::log_1(&JsString::from(resp.as_str()));
          });
        }
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
