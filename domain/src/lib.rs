use async_trait::async_trait;

#[async_trait]
trait UserRepo {
    type User;
    type UserId;

    async fn create_user(name: String) -> Self::User;
    async fn get_users() -> Vec<Self::User>;
    async fn get_user_by_id(id: Self::UserId) -> Option<Self::User>;
}
