#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UsersResponse {
    pub id: i32,
    pub nim: String,
    pub email: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SessionResponse {
    pub id: i32,
    pub session_verifier: String,
    pub user_id: i32,
}

#[macro_export]
macro_rules! response_ok {
    ($payload:expr) => {
        Ok(actix_web::HttpResponse::Ok().body(
                $crate::serde_json::json!({"success": true, "payload": $payload}).to_string()
            )
        )
    };
}

#[macro_export]
macro_rules! response_error {
    ($err:ident, $payload:expr) => {
        Err($crate::DynoError::$err($payload.to_string()))
    };
    ($payload:expr) => {
        Err($crate::DynoError::InternalServer($payload.to_string()))
    };
}
#[macro_export]
macro_rules! assert_response {
    ($matchers:expr, $msg:literal) => {
        if !$matchers {
            return $crate::response_error!($msg);
        }
    };
}

#[macro_export]
macro_rules! assert_return_err {
    ($matchers:expr, $msg:literal) => {
        if !$matchers {
            return Err($crate::DynoError::BadRequest($msg.to_string()));
        }
    };
    ($matchers:expr, $err:ident, $msg:literal) => {
        if !$matchers {
            return Err($crate::DynoError::$err($msg.to_string()));
        }
    };
}

#[macro_export]
macro_rules! else_return_err {
    ($matchers:expr, $msg:literal, $ret:ident) => {{
        let $ret(ret) = $matchers else { return Err($crate::DynoError::BadRequest($msg.to_string())); };
        ret
    }};
    ($matchers:expr, $err:ident, $msg:literal, $ret:ident) => {{
        let $ret(ret) = $matchers else { return Err($crate::DynoError::$err($msg.to_string())); };
        ret
    }};
}
