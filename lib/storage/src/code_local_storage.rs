use chrono::DateTime;
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

#[derive(Debug, PartialEq, PartialOrd)]
struct ExpiresAt(DateTime<Utc>);

use serde::de::{self, Visitor};

struct ExpiresAtVisitor;

impl<'de> Visitor<'de> for ExpiresAtVisitor {
    type Value = ExpiresAt;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an string")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ExpiresAt(
            v.parse::<DateTime<Utc>>()
                .map_err(|v| de::Error::custom(format!("{:?}", v)))?,
        ))
    }
}
impl serde::Serialize for ExpiresAt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.to_string().as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ExpiresAt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ExpiresAtVisitor)
    }
}
#[derive(Debug)]
pub struct CodeLocalStorage {
    file: PathBuf,
    codes: BTreeMap<String, Vec<(Code, ExpiresAt)>>,
}

impl CodeLocalStorage {
    pub fn store_codes(&self) -> Result<(), LocalStorageError> {
        // FIXME: This will overwrite changes, made by the server in the meantime of running the cli.
        tracing::debug! {?self.file,"store codes in file"};
        let yaml = serde_yaml::to_string(&self.codes)?;

        if self.file.exists() {
            remove_file(&self.file)?;
        }
        let mut file = File::create(&self.file)?;
        file.write_all(yaml.as_bytes())?;

        Ok(())
    }

    fn load_codes(&mut self) -> Result<(), LocalStorageError> {
        let mut file = File::open(&self.file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let val = serde_yaml::from_str(&contents)?;
        Ok(val)
    }
}

impl Storage for CodeLocalStorage {}
impl CodeStorage for CodeLocalStorage {
    fn create(config_file: &std::path::PathBuf) -> Result<Box<Self>, crate::LocalStorageError> {
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

        Ok(Box::new(storage))
    }

    fn validate_code(&self, email: &EMail, validate_code: &str) -> Result<(), LocalStorageError> {
        // FIXME: If cli creates codes, the running server does not recognize it, because they are using two different code_storages.
        // This is bad, because every new code needs a server restart.
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

        (*expires >= ExpiresAt(Utc::now()))
            .then(|| ())
            .ok_or(LocalStorageError::CodeExpired)
    }

    fn create_code(&mut self, email: &crate::EMail) -> Result<Box<String>, LocalStorageError> {
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
        let val = (Code(code.clone()), ExpiresAt(expiration));

        self.codes
            .entry(email.0.to_string())
            .or_insert_with(|| vec![])
            .push(val);

        self.store_codes()?;

        Ok(Box::new(code))
    }

    fn clean_codes(&mut self) -> Result<(), LocalStorageError> {
        self.codes.retain(|_, v| {
            v.retain(|(_, expire)| *expire < ExpiresAt(Utc::now()));

            v.len() > 0
        });

        self.store_codes()?;
        Ok(())
    }

    fn remove_code(&mut self, email: &EMail, code: &str) -> Result<(), LocalStorageError> {
        let codes = self
            .codes
            .entry(email.0.to_string())
            .or_insert_with(|| vec![]);

        codes.retain(|(iter_code, _expire)| Code(code.to_string()) != *iter_code);

        self.store_codes()?;
        Ok(())
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
