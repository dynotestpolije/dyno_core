use std::borrow::Cow;
use std::fmt::Display as FmtDisplay;

use derive_more::Display;

use crate::DynoResult;

#[cfg(feature = "backend")]
pub mod backend;

pub mod config;
// #[cfg(feature = "frontend")]
// pub mod frontend;

crate::decl_constants!(
    pub SIGN_UP_URL                 => "/auth/sign_up",
    pub SIGN_IN_URL                 => "/auth/sign_in",
    pub SIGN_OUT_URL                => "/auth/sign_out",
    pub RESET_REQUEST_URL           => "/auth/reset_request",
    pub EMAIL_OTP_URL               => "/auth/email_otp",
    pub CHANGE_PASSWORD_URL         => "/auth/change_password",
    pub COOKIE_NAME                 => "dyno_session",
    pub USER_HEADER_NAME            => "x-user-id",
    pub DECRYPT_MASTER_KEY_URL      => "/auth/decrypt",
    pub APP_USER_AGENT              => "dynotests/desktop-app"
);

#[inline(always)]
fn validate_nim(nim: &str) -> DynoResult<'_, ()> {
    let count = nim.chars().count();
    if count < 9 && count > 13 {
        return crate::validate_error!(
            "Invalid nim: the lenght of the nim must be greather than 8 or less than 13 character"
        );
    }
    match nim.chars().next().map(|x| x.is_ascii_alphabetic()) {
        Some(true) => (),
        _ => {
            return crate::validate_error!(
                "Invalid nim: the first character in the NIM is not alphabetic!"
            )
        }
    }
    Ok(())
}
#[inline(always)]
fn validate_email<'a, T>(val: T) -> DynoResult<'a, ()>
where
    T: Into<Cow<'a, str>>,
{
    let val = val.into();
    if val.is_empty() || !val.contains('@') {
        return crate::validate_error!("Invalid email: email must contains '@'");
    }
    let parts: Vec<&str> = val.rsplitn(2, '@').collect();
    let user_part = parts[1];
    let domain_part = parts[0];

    // validate the length of each part of the email, BEFORE doing the regex
    // according to RFC5321 the max length of the local part is 64 characters
    // and the max length of the domain part is 255 characters
    // https://datatracker.ietf.org/doc/html/rfc5321#section-4.5.3.1.1
    if user_part.chars().count() > 64 || domain_part.chars().count() > 255 {
        return crate::validate_error!(
            "Invalid email: lenght email must greather than 64 or 255 characters"
        );
    }

    Ok(())
}

#[inline(always)]
fn validate_password(pswd: &str) -> DynoResult<'_, ()> {
    if !pswd.is_ascii() {
        return crate::validate_error!(
            "Invalid password: Password input must be ASCII characters!",
        );
    }
    if !pswd
        .chars()
        .any(|x| x.is_ascii_punctuation() || x.is_ascii_digit() || x.is_ascii_uppercase())
    {
        return crate::validate_error!(
            "Invalid password: Password input must be contains unique and numeric character - (ex: p4sw00rd%!#$)",
        );
    }
    Ok(())
}

#[cfg_attr(
    feature = "use_serde",
    derive(serde::Deserialize, serde::Serialize, Display),
    display(
        fmt = "session {{ id:{id} verifier:{verifier}, master_key_hash:{master_key_hash:?} }}"
    )
)]
#[derive(Debug, Clone)]
pub struct Session {
    pub id: i32,
    pub verifier: String,
    pub master_key_hash: Option<String>,
}

#[cfg_attr(
    feature = "use_serde",
    derive(serde::Deserialize, serde::Serialize, Display),
    display(fmt = "registration {{ nim:{nim}, email:{email}, password:{password} }}")
)]
#[derive(Debug, Clone)]
pub struct Registration {
    pub nim: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}
impl crate::Validate for Registration {
    fn validate(&self) -> DynoResult<'_, ()> {
        validate_nim(&self.nim)?;
        validate_email(&self.email)?;
        validate_password(&self.password)?;
        if self.confirm_password != self.password {
            return crate::validate_error!(
                "Invalid confirm_password: second password is not matching with the password"
            );
        }
        Ok(())
    }
}

#[cfg_attr(
    feature = "use_serde",
    derive(serde::Deserialize, serde::Serialize, Display),
    display(fmt = "login {{ nim:{nim}, password:{password} }}")
)]
#[derive(Debug, Clone)]
pub struct Login {
    pub nim: String,
    pub password: String,
}
impl crate::Validate for Login {
    fn validate(&self) -> crate::DynoResult<'_, ()> {
        validate_nim(&self.nim)?;
        validate_password(&self.password)
    }
}

#[cfg_attr(
    feature = "use_serde",
    derive(serde::Deserialize, serde::Serialize, Display),
    display(fmt = "response_json {{ data: {data}, status: {status} }}")
)]
#[derive(Debug, Clone)]
pub struct ApiResponse<Type: FmtDisplay> {
    pub data: Type,
    pub status: String,
}

impl<Type: FmtDisplay> ApiResponse<Type> {
    pub fn status_ok(&self) -> bool {
        matches!(self.status.as_str(), "OK" | "GOOD")
    }
}
