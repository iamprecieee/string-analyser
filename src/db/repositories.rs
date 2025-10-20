use chrono::{DateTime, Utc};
use sqlx::{Error, Result, query};

use crate::{db::pool::DbPool, models::properties::AnalysedString};

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
            analysed_string_data.properties.unique_character_count,
            analysed_string_data.properties.word_count,
            serde_json::to_value(&analysed_string_data.properties.character_frequency_map).unwrap(),
            analysed_string_data.created_at.parse::<DateTime<Utc>>().unwrap()
        ).execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn exists_by_id(&self, id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"SELECT EXISTS(SELECT 1 FROM analysed_strings WHERE id = $1) as "exists!""#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.exists)
    }
}
