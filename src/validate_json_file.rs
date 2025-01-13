use std::fs::read_to_string;

use crate::{parser::parse_json, tokenizer::get_tokens};

pub fn validate_json_file(file_path: String) -> bool {
	let json_contents = read_to_string(file_path).unwrap();
	let tokens = get_tokens(json_contents);
	parse_json(tokens)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_valid_file() {
		assert_eq!(validate_json_file(String::from("tests/valid.json")), true);
	}

	#[test]
	fn test_invalid_file() {
		assert_eq!(validate_json_file(String::from("tests/invalid.json")), false);
	}
}