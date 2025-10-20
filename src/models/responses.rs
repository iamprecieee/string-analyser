use serde::Serialize;
use serde_json::Value;
use utoipa::ToSchema;

use crate::models::properties::AnalysedString;

#[derive(Debug, Serialize, ToSchema)]
pub struct GetStringsResponse {
    pub data: Vec<AnalysedString>,
    pub count: usize,
    pub filters_applied: Value,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
    pub details: Option<Value>,
    pub status: i32,
}

impl ApiErrorResponse {
    pub fn invalid_input(message: String, details: Option<Value>) -> Self {
        Self {
            code: "INVALID_INPUT".to_string(),
            message: message,
            details: match details {
                Some(val) => Some(val),
                None => None,
            },
            status: 400,
        }
    }

    pub fn not_found(message: String, details: Option<Value>) -> Self {
        Self {
            code: "NOT_FOUND".to_string(),
            message: message,
            details: match details {
                Some(val) => Some(val),
                None => None,
            },
            status: 404,
        }
    }

    pub fn conflict(message: String, details: Option<Value>) -> Self {
        Self {
            code: "CONFLICT".to_string(),
            message: message,
            details: match details {
                Some(val) => Some(val),
                None => None,
            },
            status: 409,
        }
    }

    pub fn validaton_error(message: String, details: Option<Value>) -> Self {
        Self {
            code: "VALIDATION_ERROR".to_string(),
            message: message,
            details: match details {
                Some(val) => Some(val),
                None => None,
            },
            status: 422,
        }
    }

    pub fn throttled(message: String, details: Option<Value>) -> Self {
        Self {
            code: "RATE_LIMIT_EXCEEDED".to_string(),
            message: message,
            details: match details {
                Some(val) => Some(val),
                None => None,
            },
            status: 429,
        }
    }

    pub fn internal_error(message: String, details: Option<Value>) -> Self {
        Self {
            code: "INTERNAL_SERVER_ERROR".to_string(),
            message: message,
            details: match details {
                Some(val) => Some(val),
                None => None,
            },
            status: 500,
        }
    }
}
