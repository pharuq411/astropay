use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub struct AppError {
    pub code: ErrorCode,
    pub message: String,
    pub status: StatusCode,
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    // Auth errors
    SessionRequired,
    InvalidCredentials,
    SessionExpired,
    RateLimited,
    
    // Validation errors
    InvalidPayload,
    InvalidUuid,
    
    // Business logic errors
    InvoiceNotFound,
    InvoiceExpired,
    InvoiceAlreadyPaid,
    DuplicateEmail,
    DuplicateKeys,
    
    // System errors
    DatabaseError,
    NetworkError,
    Internal,
    NotImplemented,
}

#[derive(Debug, Clone, Copy)]
pub enum AuthErrorCode {
    SessionRequired,
    InvalidCredentials,
    SessionExpired,
    RateLimited,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            ErrorCode::SessionRequired => "SESSION_REQUIRED",
            ErrorCode::InvalidCredentials => "INVALID_CREDENTIALS", 
            ErrorCode::SessionExpired => "SESSION_EXPIRED",
            ErrorCode::RateLimited => "RATE_LIMITED",
            ErrorCode::InvalidPayload => "INVALID_PAYLOAD",
            ErrorCode::InvalidUuid => "INVALID_UUID",
            ErrorCode::InvoiceNotFound => "INVOICE_NOT_FOUND",
            ErrorCode::InvoiceExpired => "INVOICE_EXPIRED",
            ErrorCode::InvoiceAlreadyPaid => "INVOICE_ALREADY_PAID",
            ErrorCode::DuplicateEmail => "DUPLICATE_EMAIL",
            ErrorCode::DuplicateKeys => "DUPLICATE_KEYS",
            ErrorCode::DatabaseError => "DATABASE_ERROR",
            ErrorCode::NetworkError => "NETWORK_ERROR",
            ErrorCode::Internal => "INTERNAL_ERROR",
            ErrorCode::NotImplemented => "NOT_IMPLEMENTED",
        };
        write!(f, "{}", code)
    }
}

impl AppError {
    pub fn bad_request(message: &str) -> Self {
        Self {
            code: ErrorCode::InvalidPayload,
            message: message.to_string(),
            status: StatusCode::BAD_REQUEST,
        }
    }

    pub fn unauthorized(message: &str) -> Self {
        Self {
            code: ErrorCode::SessionRequired,
            message: message.to_string(),
            status: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn unauthorized_code(auth_code: AuthErrorCode) -> Self {
        let (code, message) = match auth_code {
            AuthErrorCode::SessionRequired => (ErrorCode::SessionRequired, "Authentication required"),
            AuthErrorCode::InvalidCredentials => (ErrorCode::InvalidCredentials, "Invalid email or password"),
            AuthErrorCode::SessionExpired => (ErrorCode::SessionExpired, "Session has expired"),
            AuthErrorCode::RateLimited => (ErrorCode::RateLimited, "Too many attempts, please try again later"),
        };
        Self {
            code,
            message: message.to_string(),
            status: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn not_found(message: &str) -> Self {
        Self {
            code: ErrorCode::InvoiceNotFound,
            message: message.to_string(),
            status: StatusCode::NOT_FOUND,
        }
    }

    pub fn conflict(message: &str) -> Self {
        Self {
            code: ErrorCode::DuplicateEmail,
            message: message.to_string(),
            status: StatusCode::CONFLICT,
        }
    }

    pub fn not_implemented(message: &str) -> Self {
        Self {
            code: ErrorCode::NotImplemented,
            message: message.to_string(),
            status: StatusCode::NOT_IMPLEMENTED,
        }
    }

    pub fn rate_limited(message: &str) -> Self {
        Self {
            code: ErrorCode::RateLimited,
            message: message.to_string(),
            status: StatusCode::TOO_MANY_REQUESTS,
        }
    }

    pub const Internal: Self = Self {
        code: ErrorCode::Internal,
        message: String::new(),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    };
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": {
                "code": self.code.to_string(),
                "message": if self.message.is_empty() { "Internal server error" } else { &self.message }
            }
        }));
        (self.status, body).into_response()
    }
}

impl From<deadpool_postgres::PoolError> for AppError {
    fn from(_: deadpool_postgres::PoolError) -> Self {
        Self {
            code: ErrorCode::DatabaseError,
            message: "Database connection failed".to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        tracing::error!("Database error: {}", err);
        Self {
            code: ErrorCode::DatabaseError,
            message: "Database operation failed".to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<uuid::Error> for AppError {
    fn from(_: uuid::Error) -> Self {
        Self {
            code: ErrorCode::InvalidUuid,
            message: "Invalid UUID format".to_string(),
            status: StatusCode::BAD_REQUEST,
        }
    }
}