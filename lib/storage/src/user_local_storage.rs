use config::read_config;
use serde_yaml::Value;
use std::{
    fs::{create_dir_all, remove_dir_all, remove_file, File},
    io::{Read, Write},
    path::PathBuf,
};

use crate::{local_storage::LocalStorageError, EMail, Storage, UserFile, UserProfile, UserStorage};

#[derive(Debug)]
pub struct UserLocalStorage {
    dir: PathBuf,
}

fn get_user_folder(mut dir: PathBuf, email: &EMail) -> PathBuf {
    dir.push(&email.0);
    dir
}
fn get_user_profile(dir: PathBuf, email: &EMail) -> PathBuf {
    let mut dir = get_user_folder(dir, email);
    dir.push(".userprofile");
    dir
}

impl Storage for UserLocalStorage {}
impl UserStorage for UserLocalStorage {
    fn create(config_file: &PathBuf) -> Result<Box<Self>, LocalStorageError> {
        let config = read_config(config_file)?;

        let storage = UserLocalStorage {
            dir: PathBuf::from(config.api.data_dir),
        };

        Ok(Box::new(storage))
    }

    fn get_user(&self, email: &EMail) -> Result<Box<dyn UserFile>, LocalStorageError> {
        let userprofile = get_user_profile(self.dir.clone(), &email);
        tracing::debug! {?userprofile,"get user profile"};

        let mut file = File::open(userprofile)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let val: Value = serde_yaml::from_str(&contents)?;
        Ok(Box::new(UserProfile::from_yaml(val)?))
    }

    fn create_user(
        &self,
        email: &EMail,
        password: &str,
        is_admin: &bool,
        sync15: &bool,
    ) -> Result<Box<dyn UserFile>, LocalStorageError> {
        tracing::debug! {?email,"Try to create new user"};

        let user = UserProfile::new(email.clone(), password.to_string(), *is_admin, *sync15);
        let folder = get_user_folder(self.dir.clone(), &email);

        if !folder.exists() {
            tracing::debug! {?folder,"Folder for user or parents not exists"};
            create_dir_all(folder)?;
        }

        let profile = get_user_profile(self.dir.clone(), email);
        if !profile.exists() {
            let mut file = File::create(profile)?;
            file.write_all(user.to_yaml().as_bytes())?;
            println!("User created");
            Ok(Box::new(user))
        } else {
            Err(LocalStorageError::UserAlreadyExists)
        }
    }

    fn delete_user(&self, email: &EMail) -> Result<(), LocalStorageError> {
        let folder = get_user_folder(self.dir.clone(), &email);
        tracing::debug! {?folder, "delete user"};
        remove_dir_all(folder)?;
        println!("User removed");
        Ok(())
    }

    fn edit_user(
        &self,
        email: &EMail,
        password: &str,
        is_admin: &bool,
        sync15: &bool,
    ) -> Result<(), LocalStorageError> {
        let userprofile = get_user_profile(self.dir.clone(), &email);
        tracing::debug! {?userprofile, "edit user"};
        remove_file(userprofile)?;
        self.create_user(email, password, is_admin, sync15)?;

        println!("User edited");
        Ok(())
    }
}
