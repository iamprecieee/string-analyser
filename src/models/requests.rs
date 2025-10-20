use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateStringRequest {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct NlpQuery {
    pub query: String,
}
