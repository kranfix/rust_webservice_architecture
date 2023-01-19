use domain::{async_trait, UserRepo};

#[derive(Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
}

struct UserDB(Vec<User>);

#[async_trait]
impl UserRepo for UserDB {
    type User = User;
    type UserId = u32;

    async fn create_user(&mut self, name: String) -> Self::User {
        let user = User {
            id: self.0.len() as u32 + 1,
            name,
        };
        self.0.push(user.clone());
        user
    }

    async fn get_users(&self) -> Vec<Self::User> {
        self.0.clone()
    }

    async fn get_user_by_id(&self, id: Self::UserId) -> Option<Self::User> {
        if id == 0 {
            return None;
        }
        self.0.get(id as usize - 1).cloned()
    }
}
