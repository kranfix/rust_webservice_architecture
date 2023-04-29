use domain::{
  async_trait, CreateUserError, DeleteUserError, GetUsersByIdError, GetUsersError, UpdateUserError,
  UserRepo,
};
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct UserDB(Mutex<InnerUserDB>);

#[derive(Default)]
struct InnerUserDB {
  len: usize,
  data: HashMap<String, User>,
}

impl InnerUserDB {
  fn create_user(&mut self, name: String) -> Result<User, CreateUserError> {
    let user = User {
      id: (self.len as u64 + 1).to_string(),
      nick: name,
    };
    self.data.insert(user.id.clone(), user.clone());
    self.len += 1;

    Ok(user)
  }

  fn get_users(&self) -> Result<Vec<User>, GetUsersError> {
    Ok(self.data.values().cloned().collect())
  }

  fn get_user_by_id(&self, id: String) -> Result<User, GetUsersByIdError> {
    self
      .data
      .get(&id)
      .cloned()
      .ok_or(GetUsersByIdError::NotFound(id))
  }

  fn delete_user(&mut self, id: String) -> Result<User, DeleteUserError> {
    let deleted_user = self
      .data
      .remove(&id)
      .ok_or(DeleteUserError::UserNotFound(id))?;
    Ok(deleted_user)
  }

  fn update_user(&mut self, id: String, name: String) -> Result<User, UpdateUserError> {
    let mut user = self
      .data
      .get_mut(&id)
      .ok_or(UpdateUserError::UserNotFound(id))?;
    user.nick = name;
    Ok(user.clone())
  }
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

  async fn create_user(&self, name: String) -> Result<Self::User, CreateUserError> {
    let user = {
      let mut this = self.0.lock().await;
      this.create_user(name)?
    };
    Ok(user)
  }

  async fn get_users(&self) -> Result<Vec<Self::User>, GetUsersError> {
    let users = {
      let this = self.0.lock().await;
      this.get_users()?
    };
    Ok(users)
  }

  async fn get_user_by_id(&self, id: String) -> Result<Self::User, GetUsersByIdError> {
    let user = {
      let this = self.0.lock().await;
      this.get_user_by_id(id)?
    };
    Ok(user)
  }

  async fn delete_user(&self, id: String) -> Result<Self::User, DeleteUserError> {
    let user = {
      let mut this = self.0.lock().await;
      this.delete_user(id)?
    };
    Ok(user)
  }

  async fn update_user(&self, id: String, name: String) -> Result<Self::User, UpdateUserError> {
    let user = {
      let mut this = self.0.lock().await;
      this.update_user(id, name)?
    };
    Ok(user)
  }
}
