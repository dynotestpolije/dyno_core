use std::{borrow::Cow, error::Error};

use paste::paste;

macro_rules! impl_from_to_string {
    ($structs:ty => [ $($($m:literal)? $source:ty as $kind:ident),* $(,)?]) => {
        $(
            $(#[cfg(feature = $m)])?
            impl From<$source> for $structs {
                fn from(err: $source) -> Self {
                    Self::new(err.to_string(), ErrKind::$kind)
                }
            }
        )*
    };
}
macro_rules! impl_err_kind {
    ($structs:ty => [ $( $($m:literal)? $name:ident ),* $(,)?]) => {
        impl<'a> $structs {
            paste!{$(
                $(#[cfg(feature = $m)])?
                #[allow(unused)]
                #[inline(always)]
                #[doc(hidden)]
                pub fn [<$name:snake>]<S: Into<Cow<'a, str>>>(desc: S) -> Self {
                    Self {
                        desc: desc.into(),
                        kind: ErrKind::$name,
                    }
                }
                $(#[cfg(feature = $m)])?
                #[allow(unused)]
                #[inline(always)]
                #[doc(hidden)]
                pub const fn [<is_ $name:snake>](&self) -> bool {
                    matches!(self.kind, ErrKind::$name)
                }
            )*}
        }
    };
}
#[cfg_attr(
    all(feature = "use_serde", feature = "backend"),
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Debug, Clone, Copy, derive_more::Display)]
pub enum ErrKind {
    #[cfg(feature = "backend")]
    InternalServerError,
    #[cfg(feature = "backend")]
    BadRequestError,
    #[cfg(feature = "backend")]
    UnauthorizedError,
    #[cfg(feature = "backend")]
    ForbiddenError,
    #[cfg(feature = "backend")]
    UnsupportedMediaTypeError,
    #[cfg(feature = "backend")]
    NotImplementedError,
    #[cfg(feature = "backend")]
    PasswordHashError,
    #[cfg(feature = "backend")]
    DatabaseError,

    #[cfg(feature = "frontend")]
    SendRequestError,
    #[cfg(feature = "frontend")]
    ApiError,

    #[cfg(feature = "use_excel")]
    ExcelError,

    #[cfg(feature = "use_serde")]
    SerializeError,

    #[cfg(feature = "use_serde")]
    DeserializeError,

    FilesistemError,
    InputOutputError,
    AnyError,
    UnknownError,
    ServiceError,
    SerdeError,
    ParsingError,
    EncodingDecodingError,
    ValidateError,
    Noop,
}

#[cfg_attr(
    all(feature = "use_serde", feature = "backend"),
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Debug, derive_more::Display)]
#[display(fmt = "ERROR: {kind} - {desc}")]
pub struct DynoErr<'a> {
    desc: Cow<'a, str>,
    kind: ErrKind,
}

impl_err_kind!(DynoErr<'a> => [
    "backend" InternalServerError,
    "backend" BadRequestError,
    "backend" UnauthorizedError,
    "backend" ForbiddenError,
    "backend" UnsupportedMediaTypeError,
    "backend" NotImplementedError,
    "backend" PasswordHashError,
    "backend" DatabaseError,
    "frontend" SendRequestError,
    "frontend" ApiError,
    "use_excel" ExcelError,
    "use_serde" SerializeError,
    "use_serde" DeserializeError,
    FilesistemError,
    InputOutputError,
    AnyError,
    UnknownError,
    ServiceError,
    SerdeError,
    ParsingError,
    EncodingDecodingError,
    ValidateError,
]);

impl<'a> DynoErr<'a> {
    #[inline]
    pub fn new<S: Into<Cow<'a, str>>>(desc: S, kind: ErrKind) -> Self {
        Self {
            desc: desc.into(),
            kind,
        }
    }
    #[inline]
    pub fn noop() -> Self {
        Self {
            desc: "".into(),
            kind: ErrKind::Noop,
        }
    }
}

impl Error for DynoErr<'_> {
    fn description(&self) -> &str {
        &self.desc
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
unsafe impl Send for DynoErr<'_> {}
unsafe impl Sync for DynoErr<'_> {}

impl_from_to_string!(DynoErr<'_> => [
    "use_anyhow"    anyhow::Error                                       as AnyError,
    "use_serde"     Box<bincode::ErrorKind>                             as EncodingDecodingError,
    "use_serde"     toml::de::Error                                     as DeserializeError,
    "use_serde"     toml::ser::Error                                    as SerializeError,
    "use_serde"     serde_json::Error                                   as DeserializeError,
    "use_excel"     calamine::Error                                     as ExcelError,
    "use_excel"     rust_xlsxwriter::XlsxError                          as ExcelError,
                    &'static str                                        as AnyError,
                    String                                              as AnyError,
                    Box<dyn std::error::Error>                          as AnyError,
                    Box<dyn std::error::Error + Send + Sync + 'static>  as AnyError,
                    std::io::Error                                      as InputOutputError,
                    core::num::ParseIntError                            as ParsingError,
                    core::num::ParseFloatError                          as ParsingError,
                    std::env::VarError                                  as InputOutputError,
]);

#[cfg(feature = "backend")]
impl actix_web::error::ResponseError for DynoErr<'_> {
    #[inline(always)]
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;
        match self.kind {
            ErrKind::BadRequestError => StatusCode::BAD_REQUEST,
            ErrKind::UnauthorizedError => StatusCode::UNAUTHORIZED,
            ErrKind::ForbiddenError => StatusCode::FORBIDDEN,
            ErrKind::UnsupportedMediaTypeError => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            ErrKind::NotImplementedError => StatusCode::NOT_IMPLEMENTED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    #[inline(always)]
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code())
            .json(serde_json::json!({"success": false, "payload": self}))
    }
}

pub type DynoResult<'err, T> = std::result::Result<T, DynoErr<'err>>;
