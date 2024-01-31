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
}

pub(crate) use api_error;

#[derive(Serialize, Debug, Clone)]
pub enum ApiErrorType {
    InvalidEndpoint,
    DatabaseError,
}

#[derive(Serialize, Debug, Clone)]
pub struct ApiError {
    pub code: u16,
    pub error: ApiErrorType,
}
