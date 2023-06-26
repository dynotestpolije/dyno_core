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
                #[doc = concat!(
                    "helper function to create an error with `ErrorKind::",
                    stringify!($name), "`, with parameter that implements `ToString` std traits"
                )]
                pub fn [< $name:snake _error>]<S: ToString>(desc: S) -> Self {
                    Self {
                        desc: desc.to_string(),
                        kind: ErrKind::$name,
                    }
                }

                $(#[cfg(feature = $m)])?
                #[allow(unused)]
                #[inline(always)]
                #[doc = concat!(
                    "helper function to check if the error is kind of 'ErrorKind::",
                    stringify!($name),
                    "`, return `bool` to indicate is the right kind"
                )]
                pub const fn [<is_ $name:snake _error>](&self) -> bool {
                    matches!(self.kind, ErrKind::$name)
                }
            )*}
        }
    };
}

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
    NotFound,
    #[cfg(feature = "backend")]
    ExpectationFailed,
    #[cfg(feature = "backend")]
    Database,
    #[cfg(feature = "password_hashing")]
    PasswordHash,
    #[cfg(feature = "checksum")]
    Checksum,
    #[cfg(feature = "jwt_encode_decode")]
    Jwt,
    #[cfg(feature = "use_excel")]
    Excel,
    #[cfg(feature = "use_async")]
    AsyncTask,
    Uuid,
    SendRequest,
    Api,
    Serialize,
    Deserialize,
    Plotters,
    Filesystem,
    InputOutput,
    SerialPort,
    Logger,
    Service,
    Serde,
    Parsing,
    EncodingDecoding,
    Validation,
    Any,
    Unknown,
}

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display, Debug, Clone)]
#[display(fmt = "ERROR[{kind}]: {desc}")]
pub struct DynoErr {
    pub desc: String,
    pub kind: ErrKind,
}

impl_err_kind!(DynoErr => [
    Filesystem,
    InputOutput,
    SerialPort,
    Logger,
    Service,
    Serde,
    Parsing,
    Uuid,
    Any,
    EncodingDecoding,
    Validation,
    Serialize,
    Deserialize,
    Unknown,
    SendRequest,
    Api,
    Plotters,
    "backend" InternalServer,
    "backend" BadRequest,
    "backend" Unauthorized,
    "backend" Forbidden,
    "backend" UnsupportedMediaType,
    "backend" NotImplemented,
    "backend" NotFound,
    "backend" Database,
    "backend" ExpectationFailed,
    "password_hashing" PasswordHash,
    "jwt_encode_decode" Jwt,
    "checksum" Checksum,
    "use_excel" Excel,
    "use_async" AsyncTask,
]);

impl DynoErr {
    #[inline]
    pub fn new<S: ToString>(desc: S, kind: ErrKind) -> Self {
        Self {
            desc: desc.to_string(),
            kind,
        }
    }
    #[inline]
    pub fn noop() -> Self {
        Self {
            desc: "".to_owned(),
            kind: ErrKind::Unknown,
        }
    }
    #[inline]
    pub const fn is_noop(&self) -> bool {
        matches!(self.kind, ErrKind::Unknown)
    }
}

impl std::error::Error for DynoErr {}
unsafe impl Send for DynoErr {}
unsafe impl Sync for DynoErr {}

impl_from_to_string!(DynoErr => [
    "use_excel"     calamine::Error                                     as Excel,
    "use_excel"     rust_xlsxwriter::XlsxError                          as Excel,
    "use_async"     tokio::task::JoinError                              as AsyncTask,
                    uuid::Error                                         as Uuid,
                    Box<bincode::Error>                                 as EncodingDecoding,
                    toml::de::Error                                     as Deserialize,
                    toml::ser::Error                                    as Serialize,
                    serde_json::Error                                   as Deserialize,
                    core::num::ParseIntError                            as Parsing,
                    core::num::ParseFloatError                          as Parsing,
                    std::io::Error                                      as InputOutput,
                    std::env::VarError                                  as InputOutput,
                    &'static str                                        as Any,
                    String                                              as Any,
                    Box<dyn std::error::Error>                          as Any,
                    Box<dyn std::error::Error + Send + Sync + 'static>  as Any,
]);

pub type DynoResult<T> = ::core::result::Result<T, DynoErr>;

#[cfg(feature = "backend")]
impl actix_web::error::ResponseError for DynoErr {
    #[inline]
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;
        match self.kind {
            ErrKind::BadRequest => StatusCode::BAD_REQUEST,
            ErrKind::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrKind::Forbidden => StatusCode::FORBIDDEN,
            ErrKind::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            ErrKind::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            ErrKind::NotFound => StatusCode::NOT_FOUND,
            ErrKind::InternalServer => StatusCode::INTERNAL_SERVER_ERROR,
            ErrKind::ExpectationFailed => StatusCode::EXPECTATION_FAILED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    #[inline]
    fn error_response(&self) -> actix_web::HttpResponse {
        log::error!("Error Response: {}", &self);
        actix_web::HttpResponse::build(self.status_code()).body(self.desc.clone())
    }
}

pub trait ResultHandler<T, E> {
    fn dyn_err(self) -> DynoResult<T>;
    fn ignore(self);
}

impl<T, E> ResultHandler<T, E> for ::core::result::Result<T, E>
where
    T: Sized,
    E: std::error::Error,
    DynoErr: From<E>,
{
    #[inline(always)]
    fn dyn_err(self) -> DynoResult<T> {
        match self {
            Ok(res) => Ok(res),
            Err(err) => Err(DynoErr::from(err)),
        }
    }

    #[inline(always)]
    fn ignore(self) {
        if let Err(err) = self {
            log::trace!("ERROR: {err} [ignored]")
        }
    }
}

#[macro_export]
macro_rules! ignore_err {
    ($err:expr) => {
        if let Err(err) = $err {
            $crate::log::trace!("ERROR[IGNORED]: {err}");
        }
    };
}
