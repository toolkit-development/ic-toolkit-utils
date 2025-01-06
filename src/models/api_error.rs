use std::fmt::{self, Display};

use candid::CandidType;
use ic_cdk::api::time;
use serde::{Deserialize, Serialize};

use super::validation::ValidationResponse;

#[derive(Clone, CandidType, Debug, Serialize, Deserialize)]
pub struct ApiError {
    tag: Option<String>,
    message: Option<String>,
    method_name: Option<String>,
    error_type: ApiErrorType,
    info: Option<Vec<String>>,
    timestamp: u64,
}

impl ApiError {
    pub fn new(error_type: ApiErrorType, message: Option<&str>) -> Self {
        ApiError {
            tag: None,
            message: message.map(|m| m.to_string()),
            method_name: None,
            error_type,
            info: None,
            timestamp: time(),
        }
    }

    pub fn validation_response(validation_response: Vec<ValidationResponse>) -> Self {
        Self::new(
            ApiErrorType::ValidationError(Box::new(validation_response)),
            None,
        )
    }

    pub fn not_implemented(message: Option<&str>) -> Self {
        Self::new(ApiErrorType::NotImplemented, message)
    }

    pub fn serialize(message: Option<&str>) -> Self {
        Self::new(ApiErrorType::SerializeError, message)
    }

    pub fn deserialize(message: Option<&str>) -> Self {
        Self::new(ApiErrorType::DeserializeError, message)
    }

    pub fn not_found(message: Option<&str>) -> Self {
        Self::new(ApiErrorType::NotFound, message)
    }

    pub fn bad_request(message: Option<&str>) -> Self {
        Self::new(ApiErrorType::BadRequest, message)
    }

    pub fn unauthorized(message: Option<&str>) -> Self {
        Self::new(ApiErrorType::Unauthorized, message)
    }

    pub fn forbidden(message: Option<&str>) -> Self {
        Self::new(ApiErrorType::Forbidden, message)
    }

    pub fn conflict(message: Option<&str>) -> Self {
        Self::new(ApiErrorType::Conflict, message)
    }

    pub fn external_service_error(message: Option<&str>) -> Self {
        Self::new(ApiErrorType::ExternalServiceError, message)
    }

    pub fn payload_too_large(message: Option<&str>) -> Self {
        Self::new(ApiErrorType::PayloadTooLarge, message)
    }

    pub fn service_unavailable(message: Option<&str>) -> Self {
        Self::new(ApiErrorType::ServiceUnavailable, message)
    }

    pub fn unexpected(message: Option<&str>) -> Self {
        Self::new(ApiErrorType::Unexpected, message)
    }

    pub fn unsupported(message: Option<&str>) -> Self {
        Self::new(ApiErrorType::Unsupported, message)
    }

    pub fn duplicate(message: Option<&str>) -> Self {
        Self::new(ApiErrorType::Duplicate, message)
    }

    pub fn deprecated(message: Option<&str>) -> Self {
        Self::new(ApiErrorType::Deprecated, message)
    }

    pub fn add_tag<S: Display>(mut self, tag: S) -> Self {
        self.tag = Some(tag.to_string());
        self
    }

    #[deprecated]
    pub fn add_message<S: Display>(mut self, message: S) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn add_info<S: Display>(mut self, info: S) -> Self {
        let mut info_vec = self.info.unwrap_or_default();
        info_vec.push(info.to_string());
        self.info = Some(info_vec);
        self
    }

    pub fn add_method_name<S: Display>(mut self, method_name: S) -> Self {
        self.method_name = Some(method_name.to_string());
        self
    }
}

#[derive(Clone, CandidType, Debug, Deserialize, Serialize)]
pub enum ApiErrorType {
    NotImplemented,
    Unexpected,
    Unauthorized,
    NotFound,
    BadRequest,
    Unsupported,
    Duplicate,
    ValidationError(Box<Vec<ValidationResponse>>),
    SerializeError,
    DeserializeError,
    PayloadTooLarge,
    ServiceUnavailable,
    Conflict,
    Forbidden,
    ExternalServiceError,
    Deprecated,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ApiError: tag: {:?}, message: {:?}, method_name: {:?}, error_type: {:?}, info: {:?}",
            self.tag, self.message, self.method_name, self.error_type, self.info
        )
    }
}

impl fmt::Display for ApiErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ApiErrorType::*;
        match self {
            NotImplemented => write!(f, "NotImplemented"),
            Unexpected => write!(f, "Unexpected"),
            Unauthorized => write!(f, "Unauthorized"),
            NotFound => write!(f, "NotFound"),
            BadRequest => write!(f, "BadRequest"),
            Unsupported => write!(f, "Unsupported"),
            Duplicate => write!(f, "Duplicate"),
            ValidationError(_) => write!(f, "ValidationError"),
            SerializeError => write!(f, "SerializeError"),
            DeserializeError => write!(f, "DeserializeError"),
            PayloadTooLarge => write!(f, "PayloadTooLarge"),
            ServiceUnavailable => write!(f, "ServiceUnavailable"),
            Conflict => write!(f, "Conflict"),
            Forbidden => write!(f, "Forbidden"),
            ExternalServiceError => write!(f, "ExternalServiceError"),
            Deprecated => write!(f, "Deprecated"),
        }
    }
}
