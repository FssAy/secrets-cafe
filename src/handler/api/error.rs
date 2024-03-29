/// This macro is the easiest way to create api errors with a specific status code without a derive macro.
macro_rules! api_error {
    (InvalidEndpoint) => {
        crate::handler::api::error::ApiError {
            code: 404,
            error: crate::handler::api::error::ApiErrorType::InvalidEndpoint,
        }
    };

    (DatabaseError) => {
        crate::handler::api::error::ApiError {
            code: 500,
            error: crate::handler::api::error::ApiErrorType::DatabaseError,
        }
    };

    (AlreadyExists) => {
        crate::handler::api::error::ApiError {
            code: 409,
            error: crate::handler::api::error::ApiErrorType::AlreadyExists,
        }
    };

    (InvalidPassword) => {
        crate::handler::api::error::ApiError {
            code: 401,
            error: crate::handler::api::error::ApiErrorType::InvalidPassword,
        }
    };

    (InvalidSessionToken) => {
        crate::handler::api::error::ApiError {
            code: 401,
            error: crate::handler::api::error::ApiErrorType::InvalidSessionToken,
        }
    };

    (ModNotFound) => {
        crate::handler::api::error::ApiError {
            code: 404,
            error: crate::handler::api::error::ApiErrorType::ModNotFound,
        }
    };

    (MethodNotSupported) => {
        crate::handler::api::error::ApiError {
            code: 405,
            error: crate::handler::api::error::ApiErrorType::MethodNotSupported,
        }
    };

    (NetworkError) => {
        crate::handler::api::error::ApiError {
            code: 500,
            error: crate::handler::api::error::ApiErrorType::NetworkError,
        }
    };

    (InvalidBody) => {
        crate::handler::api::error::ApiError {
            code: 400,
            error: crate::handler::api::error::ApiErrorType::InvalidBody,
        }
    };

    (PostNotFound) => {
        crate::handler::api::error::ApiError {
            code: 404,
            error: crate::handler::api::error::ApiErrorType::PostNotFound,
        }
    };

    (InvalidHeader) => {
        crate::handler::api::error::ApiError {
            code: 400,
            error: crate::handler::api::error::ApiErrorType::InvalidHeader,
        }
    };

    (MissingPermission) => {
        crate::handler::api::error::ApiError {
            code: 401,
            error: crate::handler::api::error::ApiErrorType::MissingPermission,
        }
    };

    (NoPostsLeft) => {
        crate::handler::api::error::ApiError {
            code: 404,
            error: crate::handler::api::error::ApiErrorType::NoPostsLeft,
        }
    };

    (RateLimitSystemFailed) => {
        crate::handler::api::error::ApiError {
            code: 500,
            error: crate::handler::api::error::ApiErrorType::RateLimitSystemFailed,
        }
    };

    (TooManyRequests) => {
        crate::handler::api::error::ApiError {
            code: 429,
            error: crate::handler::api::error::ApiErrorType::TooManyRequests,
        }
    };
}

use std::str::FromStr;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Response, StatusCode};
pub(crate) use api_error;
use crate::database::types::TokenError;
use crate::handler::Res;

#[derive(Serialize, Debug, Clone)]
pub enum ApiErrorType {
    InvalidEndpoint,
    DatabaseError,
    AlreadyExists,
    InvalidPassword,
    InvalidSessionToken,
    ModNotFound,
    MethodNotSupported,
    NetworkError,
    InvalidBody,
    PostNotFound,
    InvalidHeader,
    MissingPermission,
    DatabaseRejectedTheRequest,
    NoPostsLeft,
    RateLimitSystemFailed,
    TooManyRequests,
}

#[derive(Serialize, Debug, Clone)]
pub struct ApiError {
    pub code: u16,
    pub error: ApiErrorType,
}

impl From<surrealdb::Error> for ApiError {
    fn from(err: surrealdb::Error) -> Self {
        use surrealdb::err::Error::*;

        #[cfg(test)] {
            println!("[ERROR] {}", err);
        }

        #[cfg(debug_assertions)] {
            error!("{}", err);
        }

        match err {
            surrealdb::Error::Db(err_db) => match err_db {
                // todo: find out which variants should return different error
                RecordExists{..} | IndexExists{..} => api_error!(AlreadyExists),
                FieldValue { value, thing, .. } => {
                    if thing.starts_with("error:") {
                        if let Ok(status) = StatusCode::from_str(value.as_str()) {
                            // todo: get the ApiErrorType from the error
                            return ApiError {
                                code: status.as_u16(),
                                error: ApiErrorType::DatabaseRejectedTheRequest,
                            };
                        }
                    }

                    api_error!(DatabaseError)
                }
                _ => api_error!(DatabaseError),
            }
            surrealdb::Error::Api(err_db_api) => match err_db_api {
                // todo: find out which variants should return different error
                _ => api_error!(DatabaseError)
            }
        }
    }
}

// Should be used only for reading the token.
// Creating the token should never really fail, so returning InvalidSessionToken is not ideal.
impl From<TokenError> for ApiError {
    fn from(_: TokenError) -> Self {
        api_error!(InvalidSessionToken)
    }
}

impl From<limtr::Error> for ApiError {
    fn from(_: limtr::Error) -> Self {
        api_error!(InvalidSessionToken)
    }

}


impl Into<Res> for ApiError {
    fn into(self) -> Res {
        let error = serde_json::to_string(&self).unwrap();
        Response::builder()
            .status(self.code)
            .body(Full::new(Bytes::from(error)))
            .unwrap()
    }
}
