use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct StringFilters {
    pub is_palindrome: Option<bool>,
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
    pub word_count: Option<i32>,
    pub contains_character: Option<String>,
}
