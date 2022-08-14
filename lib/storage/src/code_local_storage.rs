use chrono::{Duration, Utc};
use config::read_config;
use rand::Rng;
use std::{
    collections::BTreeMap,
    fs::{remove_file, File},
    io::{Read, Write},
    path::PathBuf,
};

use crate::{CodeStorage, EMail, LocalStorageError, Storage};

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
struct Code(String);

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd)]
struct ExpiresAt(String);

pub struct CodeLocalStorage {
    file: PathBuf,
    codes: BTreeMap<String, Vec<(Code, ExpiresAt)>>,
}

impl CodeLocalStorage {
    pub fn store_codes(&self) -> Result<(), LocalStorageError> {
        tracing::debug! {?self.file,"store codes in file"};
        let yaml = serde_yaml::to_string(&self.codes)?;

        if self.file.exists() {
            remove_file(&self.file)?;
        }
        let mut file = File::create(&self.file)?;
        file.write_all(yaml.as_bytes())?;

        Ok(())
    }
}

impl Storage for CodeLocalStorage {}
impl CodeStorage for CodeLocalStorage {
    fn create(config_file: &std::path::PathBuf) -> Result<Self, crate::LocalStorageError>
    where
        Self: Sized,
    {
        let config = read_config(config_file)?;

        let mut file = PathBuf::from(config.api.data_dir);
        file.push(".codes.yaml");

        let codes = match load_codes(&file) {
            Ok(v) => v,
            Err(_) => BTreeMap::new(),
        };

        let storage = CodeLocalStorage {
            file: file.clone(),
            codes,
        };

        Ok(storage)
    }

    fn validate_code(&self, email: &EMail, validate_code: &str) -> Result<(), LocalStorageError> {
        let validate_code = Code(validate_code.to_string());
        let codes = self
            .codes
            .get(&email.0)
            .ok_or(LocalStorageError::UserNotFound)?;

        let expires = &codes
            .iter()
            .filter(|(code, _)| validate_code == *code)
            .next()
            .ok_or(LocalStorageError::CodeNotValid)?
            .1;

        (*expires >= ExpiresAt(Utc::now().to_string()))
            .then(|| ())
            .ok_or(LocalStorageError::CodeExpired)
    }

    fn create_code(&mut self, email: &crate::EMail) -> Result<String, LocalStorageError> {
        const CODE_SIZE: usize = 8;
        let runes = "abcdefghijklmnopqrstuvwxyz".to_string();
        let mut rng = rand::thread_rng();

        let mut code = Vec::new();

        for _ in 0..CODE_SIZE {
            let rn: usize = rng.gen_range(0..CODE_SIZE);
            let val = &runes[rn..rn + 1];
            code.push(val);
        }

        let code = code.join("").to_uppercase();
        let expiration = Utc::now() + Duration::hours(3);
        let val = (Code(code.clone()), ExpiresAt(expiration.to_string()));

        self.codes
            .entry(email.0.to_string())
            .or_insert_with(|| vec![])
            .push(val);

        self.store_codes()?;

        Ok(code)
    }
}

fn load_codes(
    file: &PathBuf,
) -> Result<BTreeMap<String, Vec<(Code, ExpiresAt)>>, LocalStorageError> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let val = serde_yaml::from_str(&contents)?;
    Ok(val)
}
