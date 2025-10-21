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

        if (token == "exactly" || token == "equals") && index + 1 < tokens.len() {
            if let Ok(val) = tokens[index + 1].parse::<i32>() {
                filters.min_length = Some(val);
                filters.max_length = Some(val);
                index += 1;
            }
        }

        if index + 1 < tokens.len() {
            let ordinal_num = match token {
                "first" | "1st" => Some(1),
                "second" | "2nd" => Some(2),
                "third" | "3rd" => Some(3),
                "fourth" | "4th" => Some(4),
                "fifth" | "5th" => Some(5),
                "sixth" | "6th" => Some(6),
                "seventh" | "7th" => Some(7),
                "eighth" | "8th" => Some(8),
                "ninth" | "9th" => Some(9),
                "tenth" | "10th" => Some(10),
                _ => {
                    if token.ends_with("th")
                        || token.ends_with("st")
                        || token.ends_with("nd")
                        || token.ends_with("rd")
                    {
                        token
                            .trim_end_matches(|c: char| !c.is_numeric())
                            .parse::<usize>()
                            .ok()
                    } else {
                        None
                    }
                }
            };

            if ordinal_num.is_some() {
                let next = tokens[index + 1];

                let ord_num_unwrapped = ordinal_num.unwrap();

                if next == "vowel" {
                    if let Some(vowel) = {
                        let vowels = ['a', 'e', 'i', 'o', 'u'];
                        vowels.get(ord_num_unwrapped.saturating_sub(1)).copied()
                    } {
                        filters.contains_character = Some(vowel.to_string());
                        index += 1;
                    }
                } else if next == "consonant" {
                    if let Some(consonant) = {
                        let consonants = [
                            'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r',
                            's', 't', 'v', 'w', 'x', 'y', 'z',
                        ];
                        consonants.get(ord_num_unwrapped.saturating_sub(1)).copied()
                    } {
                        filters.contains_character = Some(consonant.to_string());
                        index += 1;
                    }
                } else if next == "alphabet" || next == "letter" {
                    if let Some(letter) = {
                        if ord_num_unwrapped >= 1 && ord_num_unwrapped <= 26 {
                            Some((b'a' + (ord_num_unwrapped - 1) as u8) as char)
                        } else {
                            None
                        }
                    } {
                        filters.contains_character = Some(letter.to_string());
                        index += 1;
                    }
                }
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

        if token.contains("contain") || token == "with" {
            if index + 2 < tokens.len() && (tokens[index + 1] == "the" || tokens[index + 1] == "a")
            {
                if tokens[index + 2] == "letter" || tokens[index + 2] == "character" {
                    if index + 3 < tokens.len() {
                        if let Some(ch) = tokens[index + 3].chars().next() {
                            filters.contains_character = Some(ch.to_string());
                            index += 3;
                        }
                    }
                }
            } else if index + 1 < tokens.len() {
                if let Some(ch) = tokens[index + 1].chars().next() {
                    filters.contains_character = Some(ch.to_string());
                    index += 1;
                }
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
