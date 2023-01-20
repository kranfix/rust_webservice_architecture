use domain::{async_trait, UserRepo};

pub struct UserDB(Vec<User>);

impl UserDB {
    pub fn new() -> UserDB {
        UserDB(Vec::new())
    }
}

#[derive(Clone)]
pub struct User {
    pub id: u64,
    pub nick: String,
}

impl domain::User for User {
    type Id = u32;

    fn id(&self) -> Self::Id {
        self.id as u32
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
            id: self.0.len() as u64 + 1,
            nick: name,
        };
        self.0.push(user.clone());
        user
    }

    async fn get_users(&self) -> &Vec<Self::User> {
        &self.0
    }

    async fn get_user_by_id(&self, id: <Self::User as domain::User>::Id) -> Option<Self::User> {
        if id == 0 {
            return None;
        }
        self.0.get(id as usize - 1).cloned()
    }
}
