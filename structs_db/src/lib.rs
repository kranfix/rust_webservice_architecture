use domain::{async_trait, UserRepo};
use std::collections::HashMap;

#[derive(Default)]
pub struct UserDB {
  len: usize,
  data: HashMap<String, User>,
}

#[derive(Clone)]
pub struct User {
  pub id: String,
  pub nick: String,
}

impl domain::User for User {
  fn id(&self) -> String {
    self.id.clone()
  }

  fn name(&self) -> String {
    self.nick.clone()
  }
}

#[async_trait]
impl UserRepo for UserDB {
  type User = User;

  async fn create_user(&mut self, name: String) -> Self::User {
    let user = User {
      id: (self.len as u64 + 1).to_string(),
      nick: name,
    };
    self.data.insert(user.id.clone(), user.clone());
    user
  }

  async fn get_users(&self) -> Vec<Self::User> {
    self.data.values().cloned().collect()
  }

  async fn get_user_by_id(&self, id: String) -> Option<Self::User> {
    self.data.get(&id).cloned()
  }
}
