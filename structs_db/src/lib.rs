use domain::{
  async_trait, CreateUserError, DeleteUserError, GetUsersByIdError, GetUsersError, UserRepo,
};
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

  async fn create_user(&mut self, name: String) -> Result<Self::User, CreateUserError> {
    let user = User {
      id: (self.len as u64 + 1).to_string(),
      nick: name,
    };
    self.data.insert(user.id.clone(), user.clone());
    Ok(user)
  }

  async fn get_users(&self) -> Result<Vec<Self::User>, GetUsersError> {
    Ok(self.data.values().cloned().collect())
  }

  async fn get_user_by_id(&self, id: String) -> Result<Self::User, GetUsersByIdError> {
    self
      .data
      .get(&id)
      .cloned()
      .ok_or(GetUsersByIdError::NotFound(id))
  }

  async fn delete_user(&self, id: String) -> Result<Self::User, DeleteUserError> {
    // let a = self.data.remove(&id);

    todo!()
  }
}
