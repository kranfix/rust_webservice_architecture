pub mod common_types;
pub use async_trait::async_trait;
use thiserror::Error;

#[async_trait]
pub trait UserRepo {
  type User: User;

  async fn create_user(&mut self, name: String) -> Result<Self::User, CreateUserError>;
  async fn get_users(&self) -> Result<Vec<Self::User>, GetUsersError>;
  async fn get_user_by_id(&self, id: String) -> Result<Self::User, GetUsersByIdError>;
  async fn delete_user(&mut self, id: String) -> Result<Self::User, DeleteUserError>;
  async fn update_user(&mut self, id: String, name: String) -> Result<Self::User, UpdateUserError>;
}

pub trait User: Clone {
  fn id(&self) -> String;
  fn name(&self) -> String;
}

#[derive(Error, Debug)]
pub enum CreateUserError {
  #[error("Name is bad formatted")]
  NameBadFormatted,
  #[error("Internal error :(")]
  Internal,
  // #[error("User {0} already exist")]
  // UserAlreadyExist(String),
}

#[derive(Error, Debug)]
pub enum GetUsersError {
  #[error("Internal error :(")]
  Internal,
}

#[derive(Error, Debug)]
pub enum GetUsersByIdError {
  #[error(r#"User with id="{0}" not found"#)]
  NotFound(String),
  #[error("Internal error :(")]
  Internal,
}

#[derive(Error, Debug)]
pub enum DeleteUserError {
  #[error(r#"User with id="{0}" not found"#)]
  UserNotFound(String),
  #[error("Internal error :(")]
  Internal,
}

#[derive(Error, Debug)]
pub enum UpdateUserError {
  #[error(r#"User with id="{0}" not found"#)]
  UserNotFound(String),
  #[error("Internal error :(")]
  Internal,
}
