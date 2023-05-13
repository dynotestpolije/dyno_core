use std::{borrow::Cow, error::Error};

use crate::server::ApiResponse;

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
            paste::paste!{$(
                $(#[cfg(feature = $m)])?
                #[allow(unused)]
                #[inline(always)]
                #[doc(hidden)]
                pub fn [< $name:snake _error>]<S: Into<Cow<'a, str>>>(desc: S) -> Self {
                    Self {
                        desc: desc.into(),
                        kind: ErrKind::$name,
                    }
                }

                $(#[cfg(feature = $m)])?
                #[allow(unused)]
                #[inline(always)]
                #[doc(hidden)]
                pub const fn [<is_ $name:snake _error>](&self) -> bool {
                    matches!(self.kind, ErrKind::$name)
                }
            )*}
        }
    };
}
#[derive(serde::Deserialize, serde::Serialize, derive_more::Display, Debug, Clone, Copy)]
pub enum ErrKind {
    #[cfg(feature = "backend")]
    InternalServer,
    #[cfg(feature = "backend")]
    BadRequest,
    #[cfg(feature = "backend")]
    Unauthorized,
    #[cfg(feature = "backend")]
    Forbidden,
    #[cfg(feature = "backend")]
    UnsupportedMediaType,
    #[cfg(feature = "backend")]
    NotImplemented,
    #[cfg(feature = "backend")]
    PasswordHash,
    #[cfg(feature = "backend")]
    Database,

    #[cfg(feature = "frontend")]
    SendRequest,
    #[cfg(feature = "frontend")]
    Api,

    #[cfg(feature = "use_excel")]
    Excel,

    Serialize,
    Deserialize,

    Filesistem,
    InputOutput,
    Any,
    Unknown,
    Service,
    Serde,
    Parsing,
    EncodingDecoding,
    Validate,
    Noop,
}

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display, Debug, Clone)]
#[display(fmt = "ERROR: {kind} - {desc}")]
pub struct DynoErr<'a> {
    pub desc: Cow<'a, str>,
    pub kind: ErrKind,
}

impl_err_kind!(DynoErr<'a> => [
    Filesistem, InputOutput, Any, Unknown, Service, Serde, Parsing, EncodingDecoding, Validate, Serialize, Deserialize,
    "backend" InternalServer,
    "backend" BadRequest,
    "backend" Unauthorized,
    "backend" Forbidden,
    "backend" UnsupportedMediaType,
    "backend" NotImplemented,
    "backend" PasswordHash,
    "backend" Database,
    "frontend" SendRequest,
    "frontend" Api,
    "use_excel" Excel,
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

    #[inline]
    pub fn validation<S: Into<Cow<'a, str>>>(desc: S) -> Self {
        Self {
            desc: desc.into(),
            kind: ErrKind::Validate,
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
    "use_anyhow"    anyhow::Error                                       as Any,
    "use_excel"     calamine::Error                                     as Excel,
    "use_excel"     rust_xlsxwriter::XlsxError                          as Excel,
                    Box<bincode::ErrorKind>                             as EncodingDecoding,
                    toml::de::Error                                     as Deserialize,
                    toml::ser::Error                                    as Serialize,
                    serde_json::Error                                   as Deserialize,
                    &'static str                                        as Any,
                    String                                              as Any,
                    Box<dyn std::error::Error>                          as Any,
                    Box<dyn std::error::Error + Send + Sync + 'static>  as Any,
                    std::io::Error                                      as InputOutput,
                    core::num::ParseIntError                            as Parsing,
                    core::num::ParseFloatError                          as Parsing,
                    std::env::VarError                                  as InputOutput,
]);

#[cfg(feature = "backend")]
impl actix_web::error::ResponseError for DynoErr<'_> {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;
        match self.kind {
            ErrKind::BadRequest => StatusCode::BAD_REQUEST,
            ErrKind::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrKind::Forbidden => StatusCode::FORBIDDEN,
            ErrKind::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            ErrKind::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    #[inline]
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code())
            .json(ApiResponse::<Self>::error(self.clone()))
    }
}

pub type DynoResult<'err, T> = std::result::Result<T, DynoErr<'err>>;
