use serde::de::DeserializeOwned;

use crate::query_result::QueryResult;

pub struct SurrealReqwest {
  ns: String,
  db: String,
  addr: String,
  auth: Auth,
}

impl SurrealReqwest {
  pub fn new(
    ns: impl Into<String>,
    db: impl Into<String>,
    addr: impl Into<String>,
    auth: Auth,
  ) -> Self {
    Self {
      ns: ns.into(),
      db: db.into(),
      addr: addr.into(),
      auth,
    }
  }

  pub async fn sql<T: DeserializeOwned>(
    &self,
    sql: impl Into<String>,
  ) -> Result<Vec<QueryResult<T>>, String> {
    let client = reqwest::Client::new();
    let reqwest = client
      .post(format!("{}/sql", self.addr))
      .basic_auth(self.auth.user.clone(), Some(self.auth.pass.clone()))
      .header("NS", self.ns.clone())
      .header("DB", self.db.clone())
      .header("Accept", "application/json")
      .body(sql.into());
    reqwest
      .send()
      .await
      .map_err(|e| format!("HTTP ERROR: {e:?}"))?
      .json::<Vec<QueryResult<T>>>()
      .await
      .map_err(|e| format!("QueryResult parse error: {e:?}"))
  }
}

pub struct Auth {
  user: String,
  pass: String,
}

impl Auth {
  pub fn new(user: impl Into<String>, pass: impl Into<String>) -> Auth {
    Auth {
      user: user.into(),
      pass: pass.into(),
    }
  }
}
