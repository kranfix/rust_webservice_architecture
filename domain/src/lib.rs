pub use async_trait::async_trait;

#[async_trait]
pub trait UserRepo {
  type User: User;

  async fn create_user(&mut self, name: String) -> Self::User;
  async fn get_users(&self) -> Vec<Self::User>;
  async fn get_user_by_id(&self, id: String) -> Option<Self::User>;
}

pub trait User: Clone {
  fn id(&self) -> String;
  fn name(&self) -> String;
}
