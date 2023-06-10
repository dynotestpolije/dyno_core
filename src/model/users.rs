use super::role::Roles;
use crate::{DynoErr, DynoResult};
use chrono::NaiveDateTime;

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display, Debug, Clone)]
#[display(fmt = "UserResponse {{ nim:{nim}, name:{name}, email:{email:?}, role:{role} }}")]
pub struct UserResponse {
    pub id: i64,
    pub uuid: uuid::Uuid,
    pub nim: String,
    pub name: String,
    pub email: Option<String>,
    pub photo: Option<String>,
    pub role: Roles,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

#[derive(
    serde::Deserialize, serde::Serialize, derive_more::Display, Debug, Default, Clone, PartialEq,
)]
#[display(fmt = "UserRegistration {{ nim:{nim}, email:{email:?} }}")]
pub struct UserRegistration {
    pub nim: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub role: Roles,
}

impl crate::Validate for UserRegistration {
    fn validate(&self) -> DynoResult<()> {
        crate::validate_nim(&self.nim)?;
        crate::validate_email(&self.email)?;
        crate::validate_password(&self.password)?;
        if self.confirm_password != self.password {
            return Err(DynoErr::validation_error(
                "Invalid confirm_password: second password is not matching with the password",
            ));
        }
        Ok(())
    }
}

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display, Debug, Default, Clone)]
#[display(fmt = "UserLogin {{ nim:{nim}, password:{password} }}")]
pub struct UserLogin {
    pub nim: String,
    pub password: String,
}
impl crate::Validate for UserLogin {
    fn validate(&self) -> DynoResult<()> {
        crate::validate_nim(&self.nim)?;
        crate::validate_password(&self.password)
    }
}

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display, Debug, Default, Clone)]
#[display(fmt = "UserUpdate {{ nim:{nim:?}, name:{name:?}, email:{email:?}, role:{role:?} }}")]
pub struct UserUpdate {
    pub nim: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub role: Option<Roles>,
    pub email: Option<String>,
    pub photo: Option<String>,
}
