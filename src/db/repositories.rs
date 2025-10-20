use std::collections::HashMap;

use chrono::{DateTime, SecondsFormat, Utc};
use sqlx::{query, Error, QueryBuilder, Result};

use crate::{
    db::pool::DbPool,
    models::{
        filters::StringFilters,
        properties::{AnalysedString, StringProperties},
    },
};

#[derive(Clone)]
pub struct StringRepository {
    pool: DbPool,
}

impl StringRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, analysed_string_data: &AnalysedString) -> Result<(), Error> {
        query!(
            r#"
            INSERT INTO analysed_strings (id, value, length, is_palindrome, unique_char_count, word_count, char_frequency_map, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            analysed_string_data.id,
            analysed_string_data.value,
            analysed_string_data.properties.length,
            analysed_string_data.properties.is_palindrome,
            analysed_string_data.properties.unique_characters,
            analysed_string_data.properties.word_count,
            serde_json::to_value(&analysed_string_data.properties.character_frequency_map).unwrap(),
            analysed_string_data.created_at.parse::<DateTime<Utc>>().unwrap()
        ).execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn exists_by_id(&self, id: &str) -> Result<bool, sqlx::Error> {
        let result = query!(
            r#"SELECT EXISTS(SELECT 1 FROM analysed_strings WHERE id = $1) as "exists!""#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.exists)
    }

    pub async fn get_by_value(&self, value: &str) -> Result<Option<AnalysedString>, Error> {
        let result = query!(
            r#"
            SELECT id, value, length, is_palindrome, unique_char_count, word_count, char_frequency_map, created_at
            FROM analysed_strings
            WHERE value = $1
            "#,
            value
        ).fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => {
                let char_frequency_map: HashMap<String, i32> =
                    serde_json::from_value(row.char_frequency_map).unwrap();

                Ok(Some(AnalysedString {
                    id: row.id.clone(),
                    value: row.value,
                    properties: StringProperties {
                        length: row.length,
                        is_palindrome: row.is_palindrome,
                        unique_characters: row.unique_char_count,
                        word_count: row.word_count,
                        sha256_hash: row.id,
                        character_frequency_map: char_frequency_map,
                    },
                    created_at: row.created_at.to_rfc3339_opts(SecondsFormat::Millis, true),
                }))
            }

            None => Ok(None),
        }
    }

    pub async fn filter(
        &self,
        filter_values: &StringFilters,
    ) -> Result<Vec<AnalysedString>, Error> {
        let mut query = QueryBuilder::new(
            "SELECT id, value, length, is_palindrome, unique_char_count, word_count, char_frequency_map, created_at FROM analysed_strings WHERE 1=1",
        );

        if let Some(is_palindrome) = filter_values.is_palindrome {
            query.push(" AND is_palindrome = ");
            query.push_bind(is_palindrome);
        }

        if let Some(min_length) = filter_values.min_length {
            query.push(" AND length >= ");
            query.push_bind(min_length);
        }

        if let Some(max_length) = filter_values.max_length {
            query.push(" AND length <= ");
            query.push_bind(max_length);
        }

        if let Some(word_count) = filter_values.word_count {
            query.push(" AND word_count = ");
            query.push_bind(word_count);
        }

        if let Some(ref contains_char) = filter_values.contains_character {
            if let Some(first_char) = contains_char.chars().next() {
                query.push(" AND char_frequency_map ? ");
                query.push_bind(first_char.to_lowercase().to_string());
            }
        }

        query.push(" ORDER BY created_at DESC");

        let rows = query
            .build_query_as::<(
                String,
                String,
                i32,
                bool,
                i32,
                i32,
                serde_json::Value,
                chrono::DateTime<chrono::Utc>,
            )>()
            .fetch_all(&self.pool)
            .await?;

        let results = rows
            .into_iter()
            .map(|row| {
                let char_frequency = serde_json::from_value(row.6).unwrap();

                AnalysedString {
                    id: row.0.clone(),
                    value: row.1,
                    properties: StringProperties {
                        length: row.2,
                        is_palindrome: row.3,
                        unique_characters: row.4,
                        word_count: row.5,
                        sha256_hash: row.0,
                        character_frequency_map: char_frequency,
                    },
                    created_at: row.7.to_rfc3339_opts(SecondsFormat::Millis, true),
                }
            })
            .collect();

        Ok(results)
    }

    pub async fn delete_by_value(&self, value: &str) -> Result<bool, Error> {
        let result = query!(r#"DELETE FROM analysed_strings WHERE value = $1"#, value)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
