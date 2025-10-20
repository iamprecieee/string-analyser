use serde::Serialize;

use crate::models::properties::AnalysedString;

#[derive(Debug, Serialize)]
pub struct NlpResponse {
    pub data: Vec<AnalysedString>,
    pub count: usize,
    pub interpreted_query: InterpretedQuery,
}

#[derive(Debug, Serialize)]
pub struct InterpretedQuery {
    pub original: String,
    pub parsed_filters: serde_json::Value,
}
