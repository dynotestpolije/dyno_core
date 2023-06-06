use crate::{DynoErr, DynoResult};

pub trait Validate: Sized + std::fmt::Display {
    fn validate(&self) -> DynoResult<()>;
}

#[inline(always)]
pub fn validate_nim(nim: impl AsRef<str>) -> DynoResult<()> {
    let nim = nim.as_ref();
    let count = nim.chars().count();
    if count < 9 && count > 13 {
        return Err(DynoErr::validation_error(
            "Invalid nim: the lenght of the nim must be greather than 8 or less than 13 character",
        ));
    }
    match nim.chars().next().map(|x| x.is_ascii_alphabetic()) {
        Some(true) => Ok(()),
        _ => Err(DynoErr::validation_error(
            "Invalid nim: the first character in the NIM is not alphabetic!",
        )),
    }
}

#[inline(always)]
pub fn validate_email(val: impl AsRef<str>) -> DynoResult<()> {
    let val = val.as_ref();
    if val.is_empty() || !val.contains('@') {
        return Err(DynoErr::validation_error(
            "Invalid email: email must contains '@'",
        ));
    }
    let parts: Vec<&str> = val.rsplitn(2, '@').collect();
    let user_part = parts[1];
    let domain_part = parts[0];

    // validate the length of each part of the email, BEFORE doing the regex
    // according to RFC5321 the max length of the local part is 64 characters
    // and the max length of the domain part is 255 characters
    // https://datatracker.ietf.org/doc/html/rfc5321#section-4.5.3.1.1
    if user_part.chars().count() > 64 || domain_part.chars().count() > 255 {
        return Err(DynoErr::validation_error(
            "Invalid email: lenght email must greather than 64 or 255 characters",
        ));
    }

    Ok(())
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PasswordStrength {
    #[default]
    VeryWeak,
    Weak,
    Moderate,
    Strong,
    VeryStrong,
}
impl PasswordStrength {
    pub fn new<S: AsRef<str>>(value: S) -> Self {
        let pswd = value.as_ref();
        if pswd.chars().count() < 8 {
            return PasswordStrength::VeryWeak;
        };
        let has_numeric = pswd.chars().any(|c| c.is_ascii_digit());
        let has_uppercase = pswd.chars().any(|c| c.is_uppercase());
        let has_special = pswd.chars().any(|c| c.is_ascii_punctuation());
        match (has_uppercase, has_numeric, has_special) {
            (false, false, false) => PasswordStrength::Weak,
            (true, false, false) => PasswordStrength::Moderate,
            (false, false, true) => PasswordStrength::Moderate,
            (false, true, false) => PasswordStrength::Moderate,
            (false, true, true) => PasswordStrength::Strong,
            (true, true, false) => PasswordStrength::Strong,
            (true, false, true) => PasswordStrength::Strong,
            (true, true, true) => PasswordStrength::VeryStrong,
        }
    }
}
impl PasswordStrength {
    #[inline(always)]
    pub fn desc(self) -> &'static str {
        match self {
            PasswordStrength::VeryWeak => "Password Strength: VeryWeak",
            PasswordStrength::Weak => "Password Strength: Weak",
            PasswordStrength::Moderate => "Password Strength: Moderate",
            PasswordStrength::Strong => "Password Strength: Strong",
            PasswordStrength::VeryStrong => "Password Strength: VeryStrong",
        }
    }
    pub fn percent_color(self) -> (f32, [u8; 3]) {
        match self {
            PasswordStrength::VeryWeak => (0.0, [0, 0, 0]),
            PasswordStrength::Weak => (0.25, [255, 0, 0]),
            PasswordStrength::Moderate => (0.5, [255, 255, 0]),
            PasswordStrength::Strong => (0.75, [0, 255, 0]),
            PasswordStrength::VeryStrong => (1., [0, 0, 255]),
        }
    }
}

impl std::fmt::Display for PasswordStrength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.desc())
    }
}

#[inline(always)]
pub fn validate_password(pswd: impl AsRef<str>) -> DynoResult<()> {
    let pswd = pswd.as_ref();
    if !pswd.is_ascii() {
        return Err(DynoErr::validation_error(
            "Invalid password: password input harus karakter ASCII!",
        ));
    }
    if pswd.chars().any(|x| x.is_ascii_whitespace()) {
        return Err(DynoErr::validation_error(
            "Invalid Password: password input memiliki whitespace karakter",
        ));
    }
    if pswd.chars().count() < 8 {
        return Err(DynoErr::validation_error(
            "Invalid Password: password input harus lebih dari 8 karakter",
        ));
    }
    Ok(())
}
