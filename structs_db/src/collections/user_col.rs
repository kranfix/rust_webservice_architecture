use std::{borrow::Borrow, hash::Hash};

use crate::core::{Collection, Record};

#[derive(Clone)]
pub struct User {
  pub id: String,
  pub nick: String,
}

impl Record for User {
  type Id = String;

  fn create_next_id(collection: &mut Collection<Self>) -> Self::Id {
    (collection.len() + 1).to_string()
  }

  fn id(&self) -> &Self::Id {
    &self.id
  }
}

#[derive(Default)]
pub struct UserCollection(Collection<User>);

impl std::ops::Deref for UserCollection {
  type Target = Collection<User>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl std::ops::DerefMut for UserCollection {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl UserCollection {
  pub fn update_user<Id: ?Sized>(&mut self, id: &Id, nick: String) -> Option<&User>
  where
    String: Borrow<Id>,
    Id: Hash + Eq,
  {
    self.update(id, |u| {
      u.nick = nick;
    })
  }
}
