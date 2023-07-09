use chrono::{DateTime, Utc};

use crate::DynoConfig;

pub mod dynotests;
pub mod role;
pub mod users;

crate::decl_constants!(
    pub COOKIE_NAME                 => "dyno_session",
    pub USER_HEADER_NAME            => "x-user-id",
    pub DECRYPT_MASTER_KEY_URL      => "/auth/decrypt",
    pub APP_USER_AGENT              => "dynotests/desktop-app"
);

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display)]
#[display(fmt = "UserSession {{ id:{id}, role:{role} }}")]
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UserSession {
    pub id: i64,
    pub uuid: uuid::Uuid,
    pub role: role::Roles,
}

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display)]
#[display(fmt = "session {{ sub:{sub} iat:{iat}, exp:{exp} }}")]
#[derive(Debug, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub id: String,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
}

impl TokenClaims {
    #[allow(unused)]
    pub fn new(id: impl ToString, max_age: i64, sub: impl ToString) -> Self {
        let now = chrono::Utc::now();
        let iat = now.timestamp_millis();
        let nbf = now.timestamp_millis();
        let exp = (now + chrono::Duration::minutes(max_age as _)).timestamp_millis();
        Self {
            id: id.to_string(),
            sub: sub.to_string(),
            exp,
            iat,
            nbf,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct ActiveResponse {
    pub user: Option<users::UserResponse>,
    pub dyno: Option<DynoConfig>,
    pub start: DateTime<Utc>,
}

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display)]
#[serde(rename_all = "lowercase")]
#[derive(Debug, Clone, Copy)]
pub enum ResponseStatus {
    #[display(fmt = "success")]
    Success,

    #[display(fmt = "error")]
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HistoryResponse {
    pub id: i64,
    pub user_id: i64,
    pub created_at: chrono::NaiveDateTime,
}

#[cfg_attr(debug_assert, derive(Debug))]
#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct ApiResponse<T> {
    pub payload: T,
    pub status: ResponseStatus,
}

impl<T> ApiResponse<T>
where
    T: serde::ser::Serialize,
    T: serde::de::DeserializeOwned,
{
    pub fn success(payload: T) -> Self {
        Self {
            payload,
            status: ResponseStatus::Success,
        }
    }

    pub fn error(payload: T) -> Self {
        Self {
            payload,
            status: ResponseStatus::Error,
        }
    }

    pub const fn status_ok(&self) -> bool {
        matches!(self.status, ResponseStatus::Success)
    }
}
