use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateStringRequest {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct NlpQuery {
    pub query: String,
}
