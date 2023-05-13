use crate::DynoErr;

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display, Debug, Default, Clone)]
#[display(fmt = "registration {{ nim:{nim}, email:{email} }}")]
pub struct Registration {
    pub nim: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

impl crate::Validate for Registration {
    fn validate(&self) -> crate::DynoResult<'_, ()> {
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
#[display(fmt = "login {{ nim:{nim}, password:{password} }}")]
pub struct Login {
    pub nim: String,
    pub password: String,
}
impl crate::Validate for Login {
    fn validate(&self) -> crate::DynoResult<'_, ()> {
        crate::validate_nim(&self.nim)?;
        crate::validate_password(&self.password)
    }
}

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display, Debug, Clone)]
#[display(fmt = "UserExt {{ id:{id}, nim:{nim}, name:{name}, email:{email:?}, role:{role} }}")]
pub struct UserResponse {
    pub id: usize,
    pub nim: String,
    pub name: String,
    pub role: super::role::Roles,
    pub email: Option<String>,
    pub photo: Option<std::path::PathBuf>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display, Debug, Default, Clone)]
#[display(fmt = "UserExt {{ nim:{nim:?}, name:{name:?}, email:{email:?}, role:{role:?} }}")]
pub struct UserUpdate {
    pub nim: Option<String>,
    pub name: Option<String>,
    pub role: Option<super::role::Roles>,
    pub email: Option<String>,
    pub photo: Option<std::path::PathBuf>,
}
