use std::ops::Deref;

use leptos::*;
use service_client::{client::*, UserReply};

#[derive(PartialEq, Clone)]
pub struct User {
  pub id: String,
  pub name: String,
}

#[derive(Default, PartialEq, Clone)]
pub struct UserListState {
  pub list: Vec<User>,
}

impl IntoIterator for UserListState {
  type Item = <Vec<User> as IntoIterator>::Item;

  type IntoIter = <Vec<User> as IntoIterator>::IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    self.list.into_iter()
  }
}

pub fn create_user_controller(cx: Scope) -> UserController {
  let (users, set_users) = create_signal(cx, UserListState::default());

  let client = use_context::<Memo<Client>>(cx).expect("Client must be provided");

  let add_one_action = create_action(cx, |name: &String| async move {
    // let resp = client()
    //   .user()
    //   .create_one(&service_client::CreateUserPayload { username: *name })
    //   .await;

    // match resp {
    //   Ok(user_reply) => {
    //     let user: User = user_reply.into();
    //     set_users.update(|&mut users| users.list.push(user));
    //   }
    //   Err(e) => console::log_1(&JsString::from(format!("clientError: {e:?}").as_str())),
    // }
  });

  let update_one_action = create_action(cx, |(id, name): &(String, String)| async move {
    // let resp = client()
    //   .user()
    //   .update_one(&id, &service_client::UpdateUserPayload { username: *name })
    //   .await;

    // match resp {
    //   Ok(user_reply) => {
    //     let user: User = user_reply.into();
    //     set_users.update(|&mut users| {
    //       for u in users.list.iter_mut() {
    //         if &u.id == id {
    //           *u = User {
    //             id: id.clone(),
    //             name: name.clone(),
    //           };
    //           break;
    //         }
    //       }
    //     });
    //   }
    //   Err(e) => console::log_1(&JsString::from(format!("clientError: {e:?}").as_str())),
    // }
  });

  let delete_one_action = create_action(cx, |id: &String| async move {
    // let resp = client().user().delete_one(&id).await;

    // match resp {
    //   Ok(user_reply) => {
    //     let user: User = user_reply.into();
    //     set_users.update(|&mut users| users.list.retain(|u| u.id != user.id));
    //   }
    //   Err(e) => console::log_1(&JsString::from(format!("clientError: {e:?}").as_str())),
    // }
  });

  let fetch_all_action = create_action(cx, |_: &()| async move {
    // let resp = client().user().fetch_all().await;
    // match resp {
    //   Ok(users) => {
    //     let list: Vec<User> = users.into_iter().map(|u| u.into()).collect();
    //     set_users(UserListState { list });
    //   }
    //   Err(e) => console::log_1(&JsString::from(format!("clientError: {e:?}").as_str())),
    // }
  });

  UserController {
    users,
    add_one_action,
    fetch_all_action,
    update_one_action,
    delete_one_action,
  }
}

#[derive(Clone, Copy)]
pub struct UserController {
  pub users: ReadSignal<UserListState>,
  add_one_action: Action<String, ()>,
  update_one_action: Action<(String, String), ()>,
  delete_one_action: Action<String, ()>,
  fetch_all_action: Action<(), ()>,
}

impl Deref for UserController {
  type Target = UserListState;

  fn deref(&self) -> &Self::Target {
    let u = &self.users;
    todo!()
  }
}

impl UserController {
  pub fn add_one(&self, name: String) {
    self.add_one_action.dispatch(name)
  }

  pub fn fetch_all(&self) {
    self.fetch_all_action.dispatch(())
  }

  pub fn rm(&self, id: String) {
    self.delete_one_action.dispatch(id)
  }

  pub fn update(&self, id: String, name: String) {
    self.update_one_action.dispatch((id, name))
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
