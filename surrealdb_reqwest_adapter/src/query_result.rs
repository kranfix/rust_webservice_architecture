use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Debug, Deserialize)]
#[serde(tag = "status")]
pub enum QueryResult<T = Map<String, Value>> {
  OK { time: String, result: Vec<T> },
  ERR { time: String, detail: String },
}

#[cfg(test)]
mod test {
  use serde::Deserialize;
  use serde_json::{json, Value};

  use super::QueryResult;

  #[test]
  fn query_success_test() {
    let json = json!({
      "time": "12.877299ms",
      "status": "OK",
      "result": [
        {
          "id": "person:it2e3pzjvyj4lz5e19sx",
          "name": "frank"
        }
      ]
    });
    let QueryResult::OK { result, time }: QueryResult = serde_json::from_value(json).unwrap() else {
      unreachable!()
    };
    assert_eq!(time, "12.877299ms");
    let p = &result[0];
    let id: &Value = p.get("id").unwrap();
    assert_eq!(*id, Value::String("person:it2e3pzjvyj4lz5e19sx".into()));
    let name: &Value = p.get("name").unwrap();
    assert_eq!(*name, Value::String("frank".into()));
  }

  #[test]
  fn query_rror_test() {
    let json = json!({
      "time": "1.764826ms",
      "status": "ERR",
      "detail": "Database record `person:xl75rz9eaj54ovlyo1rl` already exists"
    });
    let QueryResult::ERR { time, detail }: QueryResult = serde_json::from_value(json).unwrap() else {
      unreachable!()
    };
    assert_eq!(time, "1.764826ms");
    assert_eq!(
      detail,
      "Database record `person:xl75rz9eaj54ovlyo1rl` already exists"
    );
  }

  #[derive(Deserialize)]
  struct Person {
    id: String,
    name: String,
  }

  #[test]
  fn query_success_person_test() {
    let json = json!({
      "time": "12.877299ms",
      "status": "OK",
      "result": [
        {
          "id": "person:it2e3pzjvyj4lz5e19sx",
          "name": "frank"
        }
      ]
    });
    let QueryResult::OK { result, time }: QueryResult<Person> = serde_json::from_value(json).unwrap() else {
      unreachable!()
    };
    assert_eq!(time, "12.877299ms");
    let p = &result[0];
    assert_eq!(p.id, "person:it2e3pzjvyj4lz5e19sx");
    assert_eq!(p.name, "frank");
  }
}
