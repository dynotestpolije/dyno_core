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
                pub fn [< $name:snake _error>]<S: ToString>(desc: S) -> Self {
                    Self {
                        desc: desc.to_string(),
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
    PasswordHash,
    #[cfg(feature = "backend")]
    Database,

    SendRequest,
    Api,

    #[cfg(feature = "use_excel")]
    Excel,

    Serialize,
    Deserialize,

    Filesistem,
    InputOutput,
    SerialPort,
    Logger,
    Service,
    Serde,
    Parsing,
    EncodingDecoding,
    Validate,
    Unknown,
}

#[derive(serde::Deserialize, serde::Serialize, derive_more::Display, Debug, Clone)]
#[display(fmt = "ERROR: {kind} - {desc}")]
pub struct DynoErr {
    pub desc: String,
    pub kind: ErrKind,
}

impl_err_kind!(DynoErr => [
    Filesistem, InputOutput,SerialPort, Logger, Service, Serde, Parsing,
    EncodingDecoding, Validate, Serialize, Deserialize, Unknown, SendRequest, Api,
    "backend" InternalServer,
    "backend" BadRequest,
    "backend" Unauthorized,
    "backend" Forbidden,
    "backend" UnsupportedMediaType,
    "backend" NotImplemented,
    "backend" PasswordHash,
    "backend" Database,
    "use_excel" Excel,
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
}

impl std::error::Error for DynoErr {}
unsafe impl Send for DynoErr {}
unsafe impl Sync for DynoErr {}

impl_from_to_string!(DynoErr => [
    "use_anyhow"    anyhow::Error                                       as Unknown,
    "use_excel"     calamine::Error                                     as Excel,
    "use_excel"     rust_xlsxwriter::XlsxError                          as Excel,
                    Box<bincode::ErrorKind>                             as EncodingDecoding,
                    toml::de::Error                                     as Deserialize,
                    toml::ser::Error                                    as Serialize,
                    serde_json::Error                                   as Deserialize,
                    &'static str                                        as Unknown,
                    String                                              as Unknown,
                    Box<dyn std::error::Error>                          as Unknown,
                    Box<dyn std::error::Error + Send + Sync + 'static>  as Unknown,
                    std::io::Error                                      as InputOutput,
                    core::num::ParseIntError                            as Parsing,
                    core::num::ParseFloatError                          as Parsing,
                    std::env::VarError                                  as InputOutput,
                    std::sync::mpsc::SendError<Option<crate::SerialData>>       as InputOutput,
]);

#[cfg(feature = "backend")]
impl actix_web::error::ResponseError for DynoErr {
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
            .json(crate::server::ApiResponse::error(self.to_string()))
    }
}

pub type DynoResult<T> = std::result::Result<T, DynoErr>;

pub trait ResultHandler<'err, T, E> {
    fn dyn_err(self) -> DynoResult<T>;
    fn ignore(self);
}

impl<'err, T, E> ResultHandler<'err, T, E> for std::result::Result<T, E>
where
    E: std::fmt::Display,
    DynoErr: From<E>,
{
    #[inline(always)]
    fn dyn_err(self) -> std::result::Result<T, DynoErr> {
        match self {
            Ok(res) => Ok(res),
            Err(err) => Err(DynoErr::from(err)),
        }
    }

    #[inline(always)]
    fn ignore(self) {
        match self {
            Ok(_) => {}
            Err(err) => log::error!("ERROR: {err} [ignored]"),
        }
    }
}
