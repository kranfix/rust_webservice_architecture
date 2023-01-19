pub use async_trait::async_trait;

#[async_trait]
pub trait UserRepo {
    type User: Clone;
    type UserId;

    async fn create_user(&mut self, name: String) -> Self::User;
    async fn get_users(&self) -> Vec<Self::User>;
    async fn get_user_by_id(&self, id: Self::UserId) -> Option<Self::User>;
}
