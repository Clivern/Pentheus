// Copyright 2025 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use toml;

/// Represents the top-level configuration structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct Configs {
    /// A mapping of database configurations.
    pub database: HashMap<String, DatabaseConfig>,
    /// A mapping of storage configurations.
    pub storage: HashMap<String, StorageConfig>,
    /// A mapping of cron job configurations.
    pub cron: HashMap<String, CronConfig>,
    /// State configuration for backups.
    pub state: StateConfig,
}

/// Represents a single database configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// The type of database driver (e.g., "sqlite", "mysql").
    pub driver: String,
    /// The hostname or IP address of the database server.
    pub host: Option<String>,
    /// The port number for connecting to the database.
    pub port: Option<u16>,
    /// The username for database authentication.
    pub user: Option<String>,
    /// The password for database authentication.
    pub password: Option<String>,
    /// The name of the database to connect to.
    pub database: Option<String>,
    /// The file path for SQLite databases.
    pub path: Option<String>,
}

/// Represents a single storage configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    /// The type of storage driver (e.g., "local", "s3").
    pub driver: String,
    /// The file path for local storage.
    pub path: Option<String>,
    /// The name of the S3 bucket (if applicable).
    pub bucket: Option<String>,
    /// The AWS region for S3 storage (if applicable).
    pub region: Option<String>,
    /// The access key for S3 authentication (if applicable).
    pub access_key: Option<String>,
    /// The secret key for S3 authentication (if applicable).
    pub secret_key: Option<String>,
    /// Compression method for stored files (e.g., "zip").
    pub compress: Option<String>,
}

/// Represents a single cron job configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct CronConfig {
    /// The schedule for the cron job (in cron format).
    pub schedule: String,
    /// The name of the database to be used by the cron job.
    pub database: String,
    /// The storage type to be used by the cron job.
    pub storage: String,
}

/// Represents the state configuration for backups.
#[derive(Debug, Serialize, Deserialize)]
pub struct StateConfig {
    /// The type of storage used for state management.
    pub storage: String,
    /// The file path to the state file.
    pub path: String,
}

/// Error returned when a configuration file is not found.
#[derive(Debug)]
struct ConfigNotFoundError {
    message: String,
}

impl std::error::Error for ConfigNotFoundError {}

impl std::fmt::Display for ConfigNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Error returned when a configuration file is invalid.
#[derive(Debug)]
struct ConfigsInvalidError {
    message: String,
}

impl std::error::Error for ConfigsInvalidError {}

impl std::fmt::Display for ConfigsInvalidError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Loads configurations from a specified TOML file.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the TOML configuration file.
///
/// # Returns
///
/// This function returns a `Result` which is:
/// - `Ok(Configs)` if loading and parsing are successful.
/// - `Err(Box<dyn std::error::Error>)` if an error occurs (e.g., file not found or invalid format).
pub fn load_configs(path: &str) -> Result<Configs, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path).map_err(|_| {
        Box::new(ConfigNotFoundError {
            message: format!("Config file {} not found", path),
        })
    })?;

    let configs = toml::from_str(&contents).map_err(|_| {
        Box::new(ConfigsInvalidError {
            message: format!("Config file {} is invalid", path),
        })
    })?;

    Ok(configs)
}

/// Tests loading valid configurations from a TOML file.
/// Tests loading valid configurations from a TOML file.
#[test]
fn test_load_configs_valid() {
    match load_configs("config.toml") {
        Ok(config) => {
            // Check state configuration
            assert_eq!(&config.state.storage, "s3_store");
            assert_eq!(&config.state.path, "/state.json");

            // Check database configurations
            assert!(config.database.contains_key("sqlite_db_01"));
            let sqlite_db = config.database.get("sqlite_db_01").unwrap();
            assert_eq!(&sqlite_db.driver, "sqlite");
            assert_eq!(
                sqlite_db.path,
                Some("/opt/backup/app1-db.sqlite".to_string())
            );
            assert!(sqlite_db.host.is_none());
            assert!(sqlite_db.port.is_none());
            assert!(sqlite_db.user.is_none());
            assert!(sqlite_db.password.is_none());
            assert!(sqlite_db.database.is_none());

            assert!(config.database.contains_key("my_mysql_db_01"));
            let mysql_db = config.database.get("my_mysql_db_01").unwrap();
            assert_eq!(&mysql_db.driver, "mysql");
            assert_eq!(mysql_db.host, Some("mysql_host".to_string()));
            assert_eq!(mysql_db.port, Some(3306));
            assert_eq!(mysql_db.user, Some("mysql_user".to_string()));
            assert_eq!(mysql_db.password, Some("mysql_password".to_string()));
            assert_eq!(mysql_db.database, Some("my_mysql_db".to_string()));
            assert!(mysql_db.path.is_none());

            assert!(config.database.contains_key("my_postgres_db_01"));
            let postgres_db = config.database.get("my_postgres_db_01").unwrap();
            assert_eq!(&postgres_db.driver, "postgres");
            assert_eq!(postgres_db.host, Some("postgres_host".to_string()));
            assert_eq!(postgres_db.port, Some(5432));
            assert_eq!(postgres_db.user, Some("postgres".to_string()));
            assert_eq!(postgres_db.password, Some("postgres_password".to_string()));
            assert_eq!(postgres_db.database, Some("my_postgres_db".to_string()));
            assert!(postgres_db.path.is_none());

            // Check storage configurations
            assert!(config.storage.contains_key("local_store"));
            let local_store = config.storage.get("local_store").unwrap();
            assert_eq!(&local_store.driver, "local");
            assert_eq!(local_store.path, Some("/path/to/local/backup".to_string()));
            assert_eq!(local_store.compress, Some("zip".to_string()));
            assert!(local_store.bucket.is_none());
            assert!(local_store.region.is_none());
            assert!(local_store.access_key.is_none());
            assert!(local_store.secret_key.is_none());

            assert!(config.storage.contains_key("s3_store"));
            let s3_store = config.storage.get("s3_store").unwrap();
            assert_eq!(&s3_store.driver, "s3");
            assert_eq!(s3_store.bucket, Some("my-backup-bucket".to_string()));
            assert_eq!(s3_store.region, Some("us-east-1".to_string()));
            assert_eq!(s3_store.access_key, Some("my_access_key".to_string()));
            assert_eq!(s3_store.secret_key, Some("my_secret_key".to_string()));
            assert_eq!(s3_store.compress, Some("none".to_string()));

            // Check cron job configurations
            assert!(config.cron.contains_key("sqlite_db_01_cron"));
            let cron_job = config.cron.get("sqlite_db_01_cron").unwrap();
            assert_eq!(&cron_job.schedule, "5 4 * * *");
            assert_eq!(&cron_job.database, "sqlite_db_01");
            assert_eq!(&cron_job.storage, "local_store");
        }
        Err(_) => {
            panic!("Failed to load configurations!");
        }
    }
}

/// Tests loading configurations from a non-existent TOML file.
#[test]
fn test_load_configs_not_found() {
    match load_configs("config.tom") {
        Ok(_) => {
            assert_eq!(true, false);
        }
        Err(e) => {
            assert_eq!(
                e.to_string(),
                "Config file config.tom not found".to_string()
            );
        }
    }
}

/// Tests loading configurations from an invalid TOML file.
#[test]
fn test_load_configs_invalid() {
    match load_configs("Cargo.toml") {
        Ok(_) => {
            assert_eq!(true, false);
        }
        Err(e) => {
            assert_eq!(
                e.to_string(),
                "Config file Cargo.toml is invalid".to_string()
            );
        }
    }
}
