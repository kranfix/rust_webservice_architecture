#[derive(Debug)]
pub struct Email(String);

impl TryFrom<&str> for Email {
  type Error = ();

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if validate_email(value) {
      Ok(Email(value.into()))
    } else {
      Err(())
    }
  }
}

fn validate_email(email: &str) -> bool {
  use regex::Regex;
  let email_regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
  email_regex.is_match(email)
}

#[cfg(test)]
mod test {
  use super::Email;

  #[test]
  fn valid_emails() {
    Email::try_from("frank@gmail.com").expect("");
    Email::try_from("allow.asd123@gmail.com").expect("");
  }

  #[test]
  fn invalid_emails() {
    Email::try_from("$@gmail.com").unwrap_err();
    Email::try_from("allow.asd123$@gmail.com").unwrap_err();
  }
}
