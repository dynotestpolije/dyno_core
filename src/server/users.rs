use crate::{server::role::Roles, DynoErr, DynoResult};
use chrono::{DateTime, Utc};

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Debug, Default, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub nim: String,
    pub name: String,
    pub password: String,
    pub role: Roles,
    pub email: Option<String>,
    pub photo: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn update_from(&mut self, value: UserUpdate) {
        let UserUpdate {
            nim,
            name,
            role,
            email,
            photo,
        } = value;
        if let Some(v) = nim {
            self.nim = v;
        }
        if let Some(v) = name {
            self.name = v;
        }
        if let Some(v) = role {
            self.role = v;
        }
        if photo.is_some() {
            self.photo = photo;
        }
        if email.is_some() {
            self.email = email;
        }
        self.updated_at = Utc::now();
    }
}

impl From<UserResponse> for User {
    #[inline]
    fn from(value: UserResponse) -> Self {
        let UserResponse {
            id,
            nim,
            name,
            email,
            photo,
            role,
            updated_at,
            created_at,
        } = value;
        Self {
            id,
            nim,
            name,
            email,
            photo,
            role,
            updated_at,
            created_at,
            ..Default::default()
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display, Debug, Default, Clone)]
#[display(fmt = "UserRegistration {{ nim:{nim}, email:{email} }}")]
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
            return Err(DynoErr::validate_error(
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

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display, Debug, Clone)]
#[display(fmt = "UserResponse {{ id:{id}, nim:{nim}, name:{name}, email:{email:?}, role:{role} }}")]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub nim: String,
    pub name: String,
    pub role: Roles,
    pub email: Option<String>,
    pub photo: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display, Debug, Default, Clone)]
#[display(fmt = "UserUpdate {{ nim:{nim:?}, name:{name:?}, email:{email:?}, role:{role:?} }}")]
pub struct UserUpdate {
    pub nim: Option<String>,
    pub name: Option<String>,
    pub role: Option<Roles>,
    pub email: Option<String>,
    pub photo: Option<String>,
}
