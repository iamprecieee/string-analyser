use std::collections::{HashMap, HashSet};

use sha2::{Digest, Sha256};

use crate::models::properties::StringProperties;

pub fn compute_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn analyse_string(input: &str) -> StringProperties {
    StringProperties {
        length: input.len() as i32,
        is_palindrome: is_palindrome(input),
        unique_characters: get_unique_chars_count(input),
        word_count: get_word_count(input),
        sha256_hash: compute_sha256(input),
        character_frequency_map: get_char_frequency_map(input),
    }
}

fn is_palindrome(input: &str) -> bool {
    let normalised = input.to_lowercase();
    normalised == normalised.chars().rev().collect::<String>()
}

fn get_unique_chars_count(input: &str) -> i32 {
    let mut chars = HashSet::new();
    for char in input.to_lowercase().chars() {
        chars.insert(char);
    }

    chars.len() as i32
}

fn get_word_count(input: &str) -> i32 {
    input.split_whitespace().count() as i32
}

fn get_char_frequency_map(input: &str) -> HashMap<String, i32> {
    let mut frequency_map = HashMap::new();
    for char in input.to_lowercase().chars() {
        *frequency_map.entry(char.to_string()).or_insert(0) += 1;
    }

    frequency_map
}
