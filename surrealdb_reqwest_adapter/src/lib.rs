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

  async fn create_user(&self, name: String) -> Result<Self::User, CreateUserError> {
    if name.trim().is_empty() {
      return Err(CreateUserError::NameBadFormatted);
    }
    let query_results = self
      .sql::<Person>(format!(r#"CREATE person SET name="{name}""#))
      .await
      .map_err(|error| {
        log::error!("UserRepo::create_user: {:?}", error);
        CreateUserError::Internal
      })?;
    let create_result = query_results.into_iter().next().ok_or_else(|| {
      log::error!("UserRepo::create_user: response does not include any result");
      CreateUserError::Internal
    })?;
    match create_result {
      QueryResult::ERR { detail, time } => {
        log::error!(
          "UserRepo::create_user: [{},{}] result could not be formatted",
          time,
          detail
        );
        return Err(CreateUserError::Internal);
      }
      QueryResult::OK { result, .. } => match result.into_iter().next() {
        Some(created_user) => Ok(created_user),
        None => {
          let e = CreateUserError::Internal;
          log::error!("UserRepo::create_user: {}", e);
          Err(e)
        }
      },
    }
  }

  async fn get_users(&self) -> Result<Vec<Self::User>, GetUsersError> {
    let select_result = self
      .sql::<Person>("SELECT * FROM person")
      .await
      .map_err(|error| {
        log::error!("UserRepo::get_users: {:?}", error);
        GetUsersError::Internal
      })?
      .into_iter()
      .next()
      .ok_or_else(|| {
        log::error!("UserRepo::get_users: Result count is not as expected");
        GetUsersError::Internal
      })?;

    match select_result {
      QueryResult::ERR { detail, time } => {
        log::error!(
          "UserRepo::get_users: [{}, {}] result could not be formatted",
          time,
          detail
        );
        Err(GetUsersError::Internal)
      }
      QueryResult::OK { result, .. } => Ok(result),
    }
  }

  async fn get_user_by_id(&self, id: String) -> Result<Self::User, GetUsersByIdError> {
    let select_result = self
      .sql::<Person>(format!(r#"SELECT * FROM person:{id}"#))
      .await
      .map_err(|error| {
        log::error!("UserRepo::get_user_by_id: {:?}", error);
        GetUsersByIdError::Internal
      })?
      .into_iter()
      .next()
      .ok_or_else(|| {
        log::error!("UserRepo::get_user_by_id: response does not include any result");
        GetUsersByIdError::Internal
      })?;
    let person = match select_result {
      QueryResult::OK { result, .. } => result.into_iter().next(),
      QueryResult::ERR { detail, time } => {
        log::error!(
          "UserRepo::get_user_by_id: [{}, {}] result could not be formatted",
          time,
          detail
        );
        return Err(GetUsersByIdError::Internal);
      }
    };
    match person {
      None => {
        let e = GetUsersByIdError::NotFound(id);
        log::error!("UserRepo::get_user_by_id: {}", e);
        Err(e)
      }
      Some(person) => Ok(person),
    }
  }

  async fn delete_user(&self, id: String) -> Result<Self::User, DeleteUserError> {
    let delete_result = self
      .sql::<Option<Person>>(format!(r#"DELETE person:{id} RETURN before"#))
      .await
      .map_err(|error| {
        log::error!("UserRepo::delete_user: {:?}", error);
        DeleteUserError::Internal
      })?
      .into_iter()
      .next()
      .ok_or_else(|| {
        log::error!("UserRepo::delete_user: response does not include any result");
        DeleteUserError::Internal
      })?;

    let deleted_user = match delete_result {
      QueryResult::ERR { detail, time } => {
        log::error!(
          "UserRepo::delete_user: [{}, {}] result could not be formatted",
          time,
          detail
        );
        return Err(DeleteUserError::Internal);
      }
      QueryResult::OK { result, .. } => result.into_iter().next().ok_or_else(|| {
        log::error!("UserRepo::delete_user: user info was expected");
        DeleteUserError::UserNotFound(id.clone())
      })?,
    };
    match deleted_user {
      None => {
        let e = DeleteUserError::UserNotFound(id);
        log::error!("UserRepo::delete_user: {}", e);
        Err(e)
      }
      Some(u) => Ok(u),
    }
  }

  async fn update_user(&self, id: String, name: String) -> Result<Self::User, UpdateUserError> {
    let query = format!(r#"UPDATE person SET name="{name}" WHERE id="person:{id}""#);
    let update_result = self
      .sql::<Person>(query)
      .await
      .map_err(|error| {
        log::error!("UserRepo::update_user: {:?}", error);
        UpdateUserError::Internal
      })?
      .into_iter()
      .next()
      .ok_or_else(|| {
        log::error!("UserRepo::update_user: response does not include any result");
        UpdateUserError::Internal
      })?;

    let updated_user = match update_result {
      QueryResult::ERR { detail, time } => {
        log::error!(
          "{} UserRepo::update_user: result could not be formatted {}",
          time,
          detail
        );
        return Err(UpdateUserError::Internal);
      }
      QueryResult::OK { result, .. } => result.into_iter().next(),
    };

    match updated_user {
      None => {
        let e = UpdateUserError::UserNotFound(id);
        log::error!("UserRepo::update_user: {}", e);
        Err(e)
      }
      Some(updated_user) => Ok(updated_user),
    }
  }
}
