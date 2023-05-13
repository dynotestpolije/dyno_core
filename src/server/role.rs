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
#[serde(rename_all = "snake_case")]
pub enum Roles {
    #[display(fmt = "Admin")]
    Admin,
    #[display(fmt = "User")]
    User,
    #[default]
    #[display(fmt = "Guest")]
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

    pub fn as_str(self) -> &'static str {
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
