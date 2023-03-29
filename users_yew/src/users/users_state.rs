use yew::Reducible;

#[derive(PartialEq, Clone)]
pub struct User {
  pub id: String,
  pub name: String,
}

#[derive(Default, PartialEq)]
pub struct UserListState {
  pub list: Vec<User>,
}

pub enum UserListAction {
  Add(String), // name
  Rm(String),  // id
}

impl Reducible for UserListState {
  type Action = UserListAction;

  fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
    match action {
      UserListAction::Add(name) => {
        let u = User {
          id: self.list.len().to_string(),
          name,
        };

        let mut list = self.list.clone();
        list.push(u);
        Self { list }.into()
      }
      UserListAction::Rm(id) => {
        let list: Vec<User> = self.list.iter().filter(|u| u.id != id).cloned().collect();
        Self { list }.into()
      }
    }
  }
}
