use crate::elements::{OBJECT_END_TOKEN, OBJECT_START_TOKEN};

pub fn get_tokens(json_text: String) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let mut chars = json_text.chars();
    while let Some(maybe_token) = chars.next() {
        let string_maybe_token = maybe_token.to_string();
        if string_maybe_token == OBJECT_START_TOKEN || string_maybe_token == OBJECT_END_TOKEN {
            tokens.push(string_maybe_token);
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let input = String::from("");
        let result = get_tokens(input);
        assert!(result.is_empty())
    }

    #[test]
    fn test_only_object_start() {
        let input = String::from("{");
        let result = get_tokens(input);
        assert_eq!(result, vec![OBJECT_START_TOKEN])
    }

    #[test]
    fn test_only_object_end() {
        let input = String::from("}");
        let result = get_tokens(input);
        assert_eq!(result, vec![OBJECT_END_TOKEN])
    }
}
