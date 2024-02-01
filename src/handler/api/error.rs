/// This macro is the easiest way to create api errors.
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
}

pub(crate) use api_error;

#[derive(Serialize, Debug, Clone)]
pub enum ApiErrorType {
    InvalidEndpoint,
    DatabaseError,
    AlreadyExists,
    InvalidPassword,
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
                _ => api_error!(DatabaseError),
            }
            surrealdb::Error::Api(err_db_api) => match err_db_api {
                // todo: find out which variants should return different error
                _ => api_error!(DatabaseError)
            }
        }
    }
}
