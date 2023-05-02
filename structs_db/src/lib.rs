pub mod collections;
pub mod core;

use domain::{
  async_trait, CreateUserError, DeleteUserError, GetUsersByIdError, GetUsersError, UpdateUserError,
  UserRepo,
};
use tokio::sync::Mutex;

impl domain::User for collections::User {
  fn id(&self) -> String {
    self.id.clone()
  }

  fn name(&self) -> String {
    self.nick.clone()
  }
}

#[derive(Default)]
pub struct UserDB(Mutex<collections::UserCollection>);

#[async_trait]
impl UserRepo for UserDB {
  type User = collections::User;

  async fn create_user(&self, name: String) -> Result<Self::User, CreateUserError> {
    let name = name.trim();
    if name.is_empty() {
      return Err(CreateUserError::NameBadFormatted);
    }
    let user = {
      let mut col = self.0.lock().await;
      col.create(|id| collections::User {
        id,
        nick: name.to_string(),
      })
    };
    user.ok_or(CreateUserError::Internal)
  }

  async fn get_users(&self) -> Result<Vec<Self::User>, GetUsersError> {
    let users = {
      let col = self.0.lock().await;
      col.get_all()
    };
    Ok(users)
  }

  async fn get_user_by_id(&self, id: String) -> Result<Self::User, GetUsersByIdError> {
    let user = {
      let col = self.0.lock().await;
      col
        .get_by_id(&id)
        .ok_or(GetUsersByIdError::NotFound(id))?
        .clone()
    };
    Ok(user)
  }

  async fn delete_user(&self, id: String) -> Result<Self::User, DeleteUserError> {
    let user = {
      let mut col = self.0.lock().await;
      col.delete(&id)
    };
    user.ok_or(DeleteUserError::UserNotFound(id))
  }

  async fn update_user(&self, id: String, name: String) -> Result<Self::User, UpdateUserError> {
    let user = {
      let mut col = self.0.lock().await;
      col
        .update_user(&id, name)
        .ok_or(UpdateUserError::UserNotFound(id))?
        .clone()
    };
    Ok(user)
  }
}
