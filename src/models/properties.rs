use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StringProperties {
    pub length: i32,
    pub is_palindrome: bool,
    pub unique_character_count: i32,
    pub word_count: i32,
    pub sha256_hash: String,
    pub character_frequency_map: HashMap<String, i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnalysedString {
    pub id: String,
    pub value: String,
    pub properties: StringProperties,
    pub created_at: String,
}
