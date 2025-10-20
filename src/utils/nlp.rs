use crate::models::filters::StringFilters;

pub struct ParsedQuery {
    pub original: String,
    pub filters: StringFilters,
}

pub fn parse_natural_language(query: &str) -> Result<ParsedQuery, String> {
    let lowercase_query = query.to_lowercase();
    let tokens = lowercase_query.split_whitespace().collect::<Vec<&str>>();

    let mut filters = StringFilters {
        is_palindrome: None,
        min_length: None,
        max_length: None,
        word_count: None,
        contains_character: None,
    };

    let mut index = 0;
    while index < tokens.len() {
        let token = tokens[index];

        if token.contains("palindro") {
            filters.is_palindrome = Some(true);
        }

        if (token.contains("short") || token.contains("small") || token.contains("less"))
            && index + 2 < tokens.len()
            && tokens[index + 1] == "than"
        {
            if let Ok(val) = tokens[index + 2].parse::<i32>() {
                filters.max_length = Some(val - 1);
                index += 2;
            }
        }

        if (token.contains("long")
            || token.contains("big")
            || token.contains("great")
            || token.contains("more")
            || token.contains("large"))
            && index + 2 < tokens.len()
            && tokens[index + 1] == "than"
        {
            if let Ok(val) = tokens[index + 2].parse::<i32>() {
                filters.min_length = Some(val + 1);
                index += 2;
            }
        }

        if token == "exactly" || token == "equals" && index + 1 < tokens.len() {
            if let Ok(val) = tokens[index + 1].parse::<i32>() {
                filters.min_length = Some(val);
                filters.max_length = Some(val);
                index += 1;
            }
        }

        if token == "single"
            || token == "one" && index + 1 < tokens.len() && tokens[index + 1] == "word"
        {
            filters.word_count = Some(1);
            index += 1;
        } else if let Ok(val) = token.parse::<i32>() {
            if index + 1 < tokens.len()
                && (tokens[index + 1] == "word" || tokens[index + 1] == "words")
            {
                filters.word_count = Some(val);
                index += 1;
            }
        } else if token.contains("-word") {
            if let Some(num_str) = token.strip_suffix("-word") {
                if let Ok(val) = num_str.parse::<i32>() {
                    filters.word_count = Some(val);
                }
            }
        }

        if (token == "contains"
            || token == "containing"
            || token == "with"
            || token == "letter"
            || token == "character")
            && index + 1 < tokens.len()
        {
            let next_token = tokens[index + 1];
            if let Some(first_char) = next_token.chars().next() {
                filters.contains_character = Some(first_char.to_string());
                index += 1;
            }
        }

        index += 1;
    }

    if filters.is_palindrome.is_none()
        && filters.min_length.is_none()
        && filters.max_length.is_none()
        && filters.word_count.is_none()
        && filters.contains_character.is_none()
    {
        return Err("Unable to parse any valid filters".to_string());
    }

    if let (Some(min), Some(max)) = (filters.min_length, filters.max_length) {
        if min > max {
            return Err("Conflicting filters: min_length > max_length".to_string());
        }
    }

    Ok(ParsedQuery {
        original: query.to_string(),
        filters,
    })
}
