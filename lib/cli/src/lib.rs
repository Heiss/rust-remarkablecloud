use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use storage::{EMail, EMailError, LocalStorageError, UserProfile, UserStorage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CLIError {
    #[error("Args parsing failed")]
    ParseError,
    #[error("Found a command and execute it")]
    CommandFound,
    #[error("User command had an error")]
    UserCommandsError(#[from] UserCommandsError),
    #[error("Storage had an error")]
    StoragesError(#[from] StoragesError),
}

#[derive(Error, Debug)]
pub enum UserCommandsError {
    #[error("Error occurred in LocalStorage")]
    LocalStorageError(#[from] LocalStorageError),
    #[error("Error occurred in email validation")]
    EMailError(#[from] EMailError),
}

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Sets a custom config file.
    #[clap(
        short,
        long,
        value_parser,
        value_name = "FILE",
        default_value = "./config.yaml"
    )]
    pub config_path: PathBuf,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    /// All user relevant commands.
    #[clap(arg_required_else_help = true)]
    User(User),
}

#[derive(Args, Clone, Debug)]
struct User {
    #[clap(subcommand)]
    command: Option<UserCommands>,
}

#[derive(Subcommand, Clone, Debug)]
enum UserCommands {
    /// Show all informations for the given email.
    Show { email: String },
    /// Edit the user with the given email and all relevant informations.
    Edit {
        email: String,
        password: String,
        is_admin: bool,
        sync15: bool,
    },
    /// Add the user with the given informations.
    Add {
        email: String,
        password: String,
        is_admin: bool,
        sync15: bool,
    },
    /// Delete the user with given email.
    Delete { email: String },
}

pub struct Storages<U: UserStorage> {
    user_storage: U,
}
#[derive(Error, Debug)]
pub enum StoragesError {
    #[error("Error occurred in LocalStorage")]
    LocalStorageError(#[from] LocalStorageError),
}

impl<U: UserStorage> Storages<U> {
    pub fn create(config: &CliArgs) -> Result<Self, StoragesError> {
        let user_storage = U::create(&config.config_path)?;
        Ok(Self { user_storage })
    }
}

pub struct CLI {}

impl CLI {
    pub fn parse_args<U: UserStorage>() -> Result<(CliArgs, Storages<U>), CLIError> {
        // TODO: Add here the workflow to add a new user (as admin)
        let args = CliArgs::parse();

        let storages = Storages::create(&args)?;

        if let Some(cmd) = &args.command {
            match cmd {
                Commands::User(u) => u.parse(&storages)?,
            }
            return Err(CLIError::CommandFound);
        }

        Ok((args, storages))
        //   Err(CLIError::ParseError)
    }
}

impl User {
    fn parse<U: UserStorage>(&self, storages: &Storages<U>) -> Result<(), UserCommandsError> {
        if let Some(v) = &self.command {
            match v {
                UserCommands::Show { email } => self.show_user(&email, storages)?,
                UserCommands::Edit {
                    email,
                    password,
                    is_admin,
                    sync15,
                } => self.edit_user(email, password, is_admin, sync15, storages)?,
                UserCommands::Add {
                    email,
                    password,
                    is_admin,
                    sync15,
                } => self.create_user(&email, &password, is_admin, sync15, storages)?,
                UserCommands::Delete { email } => self.delete_user(email, storages)?,
            }
        };

        Ok(())
    }

    fn show_user<U: UserStorage>(
        &self,
        email: &str,
        storages: &Storages<U>,
    ) -> Result<(), UserCommandsError> {
        let user = storages
            .user_storage
            .get_user::<UserProfile>(&EMail::create(email)?)?;
        println!("User: {:?}", user);
        Ok(())
    }

    fn delete_user<U: UserStorage>(
        &self,
        email: &str,
        storages: &Storages<U>,
    ) -> Result<(), UserCommandsError> {
        storages
            .user_storage
            .delete_user::<UserProfile>(&EMail::create(email)?)?;
        Ok(())
    }

    fn create_user<U: UserStorage>(
        &self,
        email: &str,
        password: &str,
        is_admin: &bool,
        sync15: &bool,
        storages: &Storages<U>,
    ) -> Result<(), UserCommandsError> {
        let _user = storages.user_storage.create_user::<UserProfile>(
            &EMail::create(email)?,
            password,
            is_admin,
            sync15,
        )?;
        Ok(())
    }

    fn edit_user<U: UserStorage>(
        &self,
        email: &str,
        password: &str,
        is_admin: &bool,
        sync15: &bool,
        storages: &Storages<U>,
    ) -> Result<(), UserCommandsError> {
        let _user = storages.user_storage.edit_user::<UserProfile>(
            &EMail::create(email)?,
            password,
            is_admin,
            sync15,
        )?;
        Ok(())
    }
}
