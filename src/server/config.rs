use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::ResultHandler;

#[cfg_attr(feature = "use_serde", derive(serde::Deserialize))]
#[derive(Default, Clone, PartialEq, Eq)]
pub enum AuthType {
    #[default]
    Normal,
    Encrypted,
}

#[cfg_attr(feature = "use_serde", derive(serde::Deserialize))]
#[derive(Clone)]
/// The global configuration
pub struct Config {
    /// The server configuration
    pub server: ServerConfig,
    pub secret: SecretConfig,
    /// The logger configuration
    #[serde(default)]
    pub log: LogConfig,
    /// The database configuration
    pub database: DatabaseConfig,
    pub email: Option<EmailConfig>,
}

impl Config {
    #[inline(always)]
    pub fn check_environtment_variables() -> bool {
        std::env::var("CONFIG_PATH").is_err()
    }

    pub fn new<'err>() -> crate::DynoResult<'err, Self> {
        Self::from_file(std::env::var("CONFIG_FILE")?)
    }

    #[inline(always)]
    pub fn from_file<'err, P>(filename: P) -> crate::DynoResult<'err, Self>
    where
        P: AsRef<Path>,
    {
        let content = read_to_string(filename)?;
        toml::from_str(&content).dyn_err()
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "use_serde", derive(serde::Deserialize))]
pub struct SecretConfig {
    pub secret_key: String,
    pub salt: Option<String>,
    pub cert: Option<String>,
    pub key: Option<String>,
}
#[derive(Clone)]
#[cfg_attr(feature = "use_serde", derive(serde::Deserialize))]
/// The server configuration
pub struct ServerConfig {
    #[serde(default = "_default_server_url")]
    pub url: String,
    #[serde(default = "_default_domain_url")]
    pub domain_url: String,
    #[serde(default)]
    pub auth_type: AuthType,
    #[serde(default)]
    pub secure_cookie: bool,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "use_serde",
    derive(serde::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub struct LogConfig {
    pub actix_server: String,
    pub actix_web: String,

    /// The logging level of the application
    pub dyno_web_server: String,
}
impl Default for LogConfig {
    fn default() -> Self {
        Self {
            actix_server: "warn".to_owned(),
            actix_web: "warn".to_owned(),
            dyno_web_server: "error".to_owned(),
        }
    }
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "use_serde", derive(serde::Deserialize))]
pub enum DbType {
    #[default]
    Postgres,
    Sqlite,
    Mysql,
}

#[derive(Clone)]
#[cfg_attr(feature = "use_serde", derive(serde::Deserialize))]
/// The database configuration
pub struct DatabaseConfig {
    #[serde(default)]
    pub db: DbType,
    pub name: String,
    pub host: String,
    pub username: String,
    pub password: String,
}

impl DatabaseConfig {
    #[inline]
    pub fn get_database_url(&self) -> String {
        match self.db {
            DbType::Postgres => format!(
                "postgresql://{username}:{password}@{host}/{name}",
                username = self.username,
                password = self.password,
                host = self.host,
                name = self.name
            ),
            DbType::Mysql => format!(
                "mysql://${username}:${password}@${host}:3306/${name}",
                username = self.username,
                password = self.password,
                host = self.host,
                name = self.name
            ),
            DbType::Sqlite => {
                let file = format!("data/{}.sqlitedb", self.name);
                if !PathBuf::from(&file).exists() {
                    std::fs::create_dir("data").expect("cannot create folder data");
                    std::fs::write(&file, b"")
                        .unwrap_or_else(|err| panic!("cannot create file {file} - {err}"));
                }
                file
            }
        }
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "use_serde", derive(serde::Deserialize))]
pub struct EmailConfig {
    #[serde(default)]
    pub enable: bool,
    pub host: String,
    pub port: i32,
    pub username: String,
    pub password: String,
    #[serde(default = "_default_true")]
    pub tls_off: bool,
    pub reset_domain: String,
    pub reset_from_email_address: String,
}
// ## Hcaptcha for throttleing - These are test keys
// [captcha]
// hcaptcha_site_key   = "10000000-ffff-ffff-ffff-000000000001"
// hcaptcha_secret_key = "0x0000000000000000000000000000000000000000"

const fn _default_true() -> bool {
    true
}
fn _default_root_url() -> String {
    "/".to_owned()
}
fn _default_domain_url() -> String {
    "localhost".to_owned()
}
fn _default_server_url() -> String {
    "0.0.0.0:8888".to_owned()
}
