use serde::Serialize;
use utoipa::ToSchema;

use crate::models::properties::AnalysedString;

#[derive(Debug, Serialize, ToSchema)]
pub struct NlpResponse {
    pub data: Vec<AnalysedString>,
    pub count: usize,
    pub interpreted_query: InterpretedQuery,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct InterpretedQuery {
    pub original: String,
    pub parsed_filters: serde_json::Value,
}
