use std::ops::Deref;

use js_sys::JsString;
use service_client::{client::*, UserReply};
use web_sys::console;
use yew::{Reducible, UseReducerHandle};

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
  AddOne(User),
  DeleteOne(String),
  SetAll(Vec<User>),
  UpdateOne { id: String, name: String },
}

#[derive(Clone, PartialEq)]
pub struct UserController {
  state: UseReducerHandle<UserListState>,
  client: UserClient,
}

impl Deref for UserController {
  type Target = UserListState;

  fn deref(&self) -> &Self::Target {
    &self.state
  }
}

impl UserController {
  pub fn new(state: UseReducerHandle<UserListState>, client: UserClient) -> Self {
    Self { state, client }
  }

  pub fn add(&self, name: String) {
    let client = self.client.clone();
    let dispatcher = self.state.dispatcher();
    wasm_bindgen_futures::spawn_local(async move {
      let resp = client
        .create_one(&service_client::CreateUserPayload { username: name })
        .await;

      match resp {
        Ok(user_reply) => {
          let user: User = user_reply.into();
          let action = UserListAction::AddOne(user);
          dispatcher.dispatch(action);
        }
        Err(e) => console::log_1(&JsString::from(format!("clientError: {e:?}").as_str())),
      }
    });
  }

  pub fn fetch_all(&self) {
    let client = self.client.clone();
    let dispatcher = self.state.dispatcher();
    wasm_bindgen_futures::spawn_local(async move {
      let resp = client.fetch_all().await;

      match resp {
        Ok(users) => {
          let users: Vec<User> = users.into_iter().map(|u| u.into()).collect();
          let action = UserListAction::SetAll(users);
          dispatcher.dispatch(action);
        }
        Err(e) => console::log_1(&JsString::from(format!("clientError: {e:?}").as_str())),
      }
    });
  }

  pub fn rm(&self, id: String) {
    let client = self.client.clone();
    let dispatcher = self.state.dispatcher();
    wasm_bindgen_futures::spawn_local(async move {
      let resp = client.delete_one(&id).await;
      match resp {
        Ok(user_reply) => {
          let id = user_reply.id;
          let action = UserListAction::DeleteOne(id);
          dispatcher.dispatch(action);
        }
        Err(e) => console::log_1(&JsString::from(format!("clientError: {e:?}").as_str())),
      }
    });
  }

  pub fn update(&self, id: String, name: String) {
    let client = self.client.clone();
    let dispatcher = self.state.dispatcher();
    wasm_bindgen_futures::spawn_local(async move {
      let resp = client
        .update_one(
          &id,
          &service_client::UpdateUserPayload {
            username: name.clone(),
          },
        )
        .await;
      match resp {
        Ok(user_reply) => {
          let id = user_reply.id;
          let action = UserListAction::UpdateOne { id, name };
          dispatcher.dispatch(action);
        }
        Err(e) => console::log_1(&JsString::from(format!("clientError: {e:?}").as_str())),
      }
    });
  }
}

impl Reducible for UserListState {
  type Action = UserListAction;

  fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
    match action {
      UserListAction::AddOne(u) => {
        let mut list = self.list.clone();
        list.push(u);
        Self { list }.into()
      }
      UserListAction::DeleteOne(id) => {
        let list: Vec<User> = self.list.iter().filter(|u| u.id != id).cloned().collect();
        Self { list }.into()
      }
      UserListAction::SetAll(list) => Self { list }.into(),
      UserListAction::UpdateOne { id, name } => {
        let mut list = Vec::new();
        for u in &self.list {
          if u.id == id {
            list.push(User {
              id: id.clone(),
              name: name.clone(),
            });
          } else {
            list.push(u.clone());
          }
        }
        Self { list }.into()
      }
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
