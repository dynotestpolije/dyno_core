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
#[derive(Debug, Clone)]
pub struct UserSession {
    pub id: uuid::Uuid,
    pub role: role::Roles,
}

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display)]
#[display(fmt = "session {{ sub:{sub} iat:{iat}, exp:{exp} }}")]
#[derive(Debug, Clone)]
pub struct TokenClaims {
    pub sub: UserSession,
    pub iat: usize,
    pub exp: usize,
}

impl TokenClaims {
    #[allow(unused)]
    pub fn new(sub: UserSession) -> Self {
        let now = chrono::Utc::now();
        let iat = now.timestamp_millis() as usize;
        let exp = (now + chrono::Duration::minutes(120)).timestamp_millis() as usize;
        Self { sub, exp, iat }
    }
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

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display)]
#[display(fmt = "response_json {{ payload: {payload}, status: {status} }}")]
#[derive(Debug, Clone)]
pub struct ApiResponse<T: std::fmt::Display> {
    pub payload: T,
    pub status: ResponseStatus,
}

impl<T> ApiResponse<T>
where
    T: serde::ser::Serialize,
    T: serde::de::DeserializeOwned,
    T: std::fmt::Display,
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
