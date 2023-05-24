#[derive(
    serde::Deserialize,
    serde::Serialize,
    derive_more::Display,
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
#[serde(rename_all = "lowercase")]
pub enum Roles {
    #[display(fmt = "admin")]
    Admin,
    #[display(fmt = "user")]
    User,
    #[default]
    #[display(fmt = "guest")]
    Guest,
}

impl Roles {
    #[allow(unused)]
    pub const fn is_admin(self) -> bool {
        matches!(self, Self::Admin)
    }
    #[allow(unused)]
    pub const fn is_user(self) -> bool {
        matches!(self, Self::Admin)
    }
    #[allow(unused)]
    pub const fn is_guest(self) -> bool {
        matches!(self, Self::Admin)
    }
}

impl<S: AsRef<str>> From<S> for Roles {
    fn from(value: S) -> Self {
        match value.as_ref() {
            "admin" => Self::Admin,
            "user" => Self::User,
            "guest" => Self::Guest,
            _ => Self::Guest,
        }
    }
}

impl crate::AsStr<'static> for Roles {
    #[inline]
    fn as_str(&self) -> &'static str {
        match self {
            Roles::Admin => "admin",
            Roles::User => "user",
            Roles::Guest => "guest",
        }
    }
}

impl std::str::FromStr for Roles {
    type Err = String;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Self::Admin),
            "user" => Ok(Self::User),
            "guest" => Ok(Self::Guest),
            _ => Err(format!(
                "Failed to parse from string in Roles, no role for {s}"
            )),
        }
    }
}

#[cfg(feature = "backend")]
impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for Roles
where
    &'r str: sqlx::Decode<'r, sqlx::Sqlite>,
{
    fn decode(
        value: <sqlx::Sqlite as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <std::borrow::Cow<'r, str> as sqlx::Decode<sqlx::Sqlite>>::decode(value)?;
        Ok(match value.as_ref() {
            "admin" => Self::Admin,
            "user" => Self::User,
            "guest" => Self::Guest,
            _ => Self::Guest,
        })
    }
}

#[cfg(feature = "backend")]
impl<'a> sqlx::Encode<'a, sqlx::Sqlite> for Roles {
    fn encode(
        self,
        buf: &mut <sqlx::Sqlite as sqlx::database::HasArguments>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        match self {
            Roles::Admin => "admin".encode(buf),
            Roles::User => "user".encode(buf),
            Roles::Guest => "guest".encode(buf),
        }
    }

    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Sqlite as sqlx::database::HasArguments<'a>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        match self {
            Roles::Admin => "admin".encode(buf),
            Roles::User => "user".encode(buf),
            Roles::Guest => "guest".encode(buf),
        }
    }
}

#[cfg(feature = "backend")]
impl sqlx::Type<sqlx::Sqlite> for Roles {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
}
