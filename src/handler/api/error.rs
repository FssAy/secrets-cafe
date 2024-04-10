use std::str::FromStr;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Response, StatusCode};
use crate::database::types::TokenError;
use crate::handler::api::error::ApiError::AlreadyExists;
use crate::handler::Res;
use crate::utils::AnyString;

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "error", content = "data")]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum ApiError {
    InvalidEndpoint(AnyString),
    DatabaseError,
    AlreadyExists(AnyString),
    InvalidPassword,
    InvalidSessionToken,
    ModNotFound,
    MethodNotSupported,
    NetworkError,
    InvalidBody,
    PostNotFound,
    InvalidHeader(AnyString),
    MissingPermission,
    DatabaseRejectedTheRequest,
    NoPostsLeft,
    RateLimitSystemFailed,
    TooManyRequests {
        limit: u64,
    },
}

impl ApiError {
    fn get_http_code(&self) -> u16 {
        use ApiError::*;

        match self {
            InvalidPassword | InvalidSessionToken | InvalidBody | InvalidHeader{..} => 400,
            InvalidEndpoint{..} | ModNotFound | PostNotFound | NoPostsLeft => 404,
            MissingPermission => 403,
            MethodNotSupported => 405,
            AlreadyExists{..} => 409,
            TooManyRequests{..} => 429,
            DatabaseError | NetworkError | DatabaseRejectedTheRequest | RateLimitSystemFailed => 500,
            _ => 500,
        }
    }
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
                RecordExists { thing } => AlreadyExists(thing.into()),
                IndexExists{ index, .. } => AlreadyExists(index.into()),
                FieldValue { value, thing, .. } => {
                    if thing.starts_with("error:") {
                        return ApiError::DatabaseRejectedTheRequest;
                    }

                    ApiError::DatabaseError
                }
                _ => ApiError::DatabaseError,
            }
            surrealdb::Error::Api(err_db_api) => match err_db_api {
                // todo: find out which variants should return different error
                _ => ApiError::DatabaseError
            }
        }
    }
}

// Should be used only for reading the token.
// Creating the token should never really fail, so returning InvalidSessionToken is not ideal.
impl From<TokenError> for ApiError {
    fn from(_: TokenError) -> Self {
        ApiError::InvalidSessionToken
    }
}

#[cfg(feature = "rate-limits")]
impl From<limtr::Error> for ApiError {
    fn from(_: limtr::Error) -> Self {
        ApiError::RateLimitSystemFailed
    }

}

impl Into<Res> for ApiError {
    fn into(self) -> Res {
        let error = serde_json::to_string(&self).unwrap();
        Response::builder()
            .status(self.get_http_code())
            .body(Full::new(Bytes::from(error)))
            .unwrap()
    }
}
