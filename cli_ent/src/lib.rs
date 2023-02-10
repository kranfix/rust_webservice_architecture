use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use domain::UserRepo;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
  /// Turn debugging information on
  #[arg(short, long, action = clap::ArgAction::Count)]
  debug: u8,

  #[command(subcommand)]
  command: Option<UserCommand>,
}

#[derive(Subcommand)]
enum UserCommand {
  /// does testing things
  List,
  Get {
    #[arg(short, long)]
    id: String,
  },
  Create {
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    email: String,
  },
  Rm {
    #[arg(short, long)]
    id: String,
  },
}

pub async fn run_cli<UR: UserRepo>(user_repo: UR) -> Result<()> {
  let cli = Cli::parse();

  // You can see how many times a particular flag or argument occurred
  // Note, only flags can have multiple occurrences
  match cli.debug {
    0 => println!("Debug mode is off"),
    1 => println!("Debug mode is kind of on"),
    2 => println!("Debug mode is on"),
    _ => println!("Don't be crazy"),
  }

  // You can check for the existence of subcommands, and if found use their
  // matches just as you would the top level cmd
  let cmd = match cli.command {
    Some(cmd) => cmd,
    None => {
      return Ok(());
    }
  };

  match cmd {
    UserCommand::List => get_users(user_repo).await?,
    UserCommand::Get { id } => get_user_by_id(user_repo, id).await?,
    UserCommand::Create { name, .. } => create_user(user_repo, name).await?,
    UserCommand::Rm { id } => delete_user(user_repo, id).await?,
  }
  Ok(())
}

pub async fn create_user<UR: UserRepo>(mut user_repo: UR, name: String) -> Result<()> {
  let user: UserReply = user_repo.create_user(name).await?.into();
  println!("{user:?}");
  Ok(())
}

pub async fn get_users<UR: UserRepo>(user_repo: UR) -> Result<()> {
  let users: Vec<UserReply> = user_repo
    .get_users()
    .await
    .context("context")?
    .into_iter()
    .map(|u| u.into())
    .collect();
  println!("Found: {users:?}");
  Ok(())
}

pub async fn get_user_by_id<UR: UserRepo>(user_repo: UR, id: String) -> Result<()> {
  let user: UserReply = user_repo.get_user_by_id(id).await?.into();
  println!("Found: {user:?}");
  Ok(())
}

pub async fn delete_user<UR: UserRepo>(mut user_repo: UR, id: String) -> Result<()> {
  let user: UserReply = user_repo.delete_user(id).await?.into();
  println!("Deleted: {user:?}");
  Ok(())
}

// the output to our `create_user` handler
#[derive(Clone, Debug)]
pub struct UserReply {
  pub id: String,
  pub username: String,
}

impl<U: domain::User> From<U> for UserReply {
  fn from(value: U) -> Self {
    UserReply {
      id: value.id(),
      username: value.name(),
    }
  }
}
