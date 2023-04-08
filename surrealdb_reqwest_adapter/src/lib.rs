pub mod client;
pub mod query_result;

pub use client::SurrealReqwest;
pub use client::*;
use domain::{
  async_trait, CreateUserError, DeleteUserError, GetUsersByIdError, GetUsersError, UpdateUserError,
};
use query_result::QueryResult;
use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Person {
  pub id: String,
  pub name: String,
}

impl domain::User for Person {
  fn id(&self) -> String {
    self.id[7..].to_string()
  }
  fn name(&self) -> String {
    self.name.clone()
  }
}

#[async_trait]
impl domain::UserRepo for SurrealReqwest {
  type User = Person;

  async fn create_user(&mut self, name: String) -> Result<Self::User, CreateUserError> {
    if name.trim().is_empty() {
      return Err(CreateUserError::NameBadFormatted);
    }
    let query_results = self
      .sql::<Person>(format!(r#"CREATE person SET name="{name}""#))
      .await
      .map_err(|_| CreateUserError::Internal)?;
    let create_result = query_results
      .into_iter()
      .next()
      .ok_or(CreateUserError::Internal)?;
    match create_result {
      QueryResult::OK { result, .. } => {
        Ok(result.into_iter().next().ok_or(CreateUserError::Internal)?)
      }
      QueryResult::ERR { .. } => return Err(CreateUserError::Internal),
    }
  }

  async fn get_users(&self) -> Result<Vec<Self::User>, GetUsersError> {
    let select_result = self
      .sql::<Person>("SELECT * FROM person")
      .await
      .map_err(|_| GetUsersError::Internal)?
      .into_iter()
      .next()
      .ok_or(GetUsersError::Internal)?;

    match select_result {
      QueryResult::OK { result, .. } => Ok(result),
      QueryResult::ERR { .. } => Err(GetUsersError::Internal),
    }
  }

  async fn get_user_by_id(&self, id: String) -> Result<Self::User, GetUsersByIdError> {
    let select_result = self
      .sql::<Person>(format!(r#"SELECT * FROM person:{id}"#))
      .await
      .map_err(|_| GetUsersByIdError::Internal)?
      .into_iter()
      .next()
      .ok_or(GetUsersByIdError::Internal)?;
    let person = match select_result {
      QueryResult::OK { result, .. } => result.into_iter().next(),
      QueryResult::ERR { .. } => return Err(GetUsersByIdError::Internal),
    };
    Ok(person.ok_or(GetUsersByIdError::NotFound(id))?)
  }

  async fn delete_user(&mut self, id: String) -> Result<Self::User, DeleteUserError> {
    let delete_result = self
      .sql::<Person>(format!(r#"DELETE person:{id} RETURN before"#))
      .await
      .map_err(|_| DeleteUserError::Internal)?
      .into_iter()
      .next()
      .ok_or(DeleteUserError::Internal)?;

    let deleted_user = match delete_result {
      QueryResult::OK { result, .. } => result.into_iter().next(),
      QueryResult::ERR { .. } => return Err(DeleteUserError::Internal),
    };

    Ok(deleted_user.ok_or(DeleteUserError::UserNotFound(id))?)
  }

  async fn update_user(&mut self, id: String, name: String) -> Result<Self::User, UpdateUserError> {
    // TODO: check if the user does not exist and throw the UpdateUserError:UserNotFound
    let update_result = self
      .sql::<Person>(format!(r#"UPDATE person:{id} SET name="{name}""#))
      .await
      .map_err(|_| UpdateUserError::Internal)?
      .into_iter()
      .next()
      .ok_or(UpdateUserError::Internal)?;

    let updated_user = match update_result {
      QueryResult::OK { result, .. } => result.into_iter().next(),
      QueryResult::ERR { .. } => return Err(UpdateUserError::Internal),
    };

    Ok(updated_user.ok_or(UpdateUserError::UserNotFound(id))?)
  }
}
