use std::slice::Iter;

use crate::elements::{OBJECT_END_TOKEN, OBJECT_START_TOKEN};

pub fn parse_json(tokens: Vec<String>) -> bool {
	let mut is_valid = false;

	let mut tokens_iter = tokens.iter();

	while let Some(token) = tokens_iter.next() {
		if token == OBJECT_START_TOKEN {
			is_valid = parse_object(&mut tokens_iter);
		}
	}

	is_valid
}

fn parse_object(tokens_iter: &mut Iter<String>) -> bool {
	let mut is_valid = false;
	while let Some(token) = tokens_iter.next() {
		if token == OBJECT_END_TOKEN {
			is_valid = true;
		}
	}
	is_valid
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_empty_tokens() {
		let tokens: Vec<String> = vec![];
		let result = parse_json(tokens);
		assert_eq!(result, false);

	}

	#[test]
	fn test_start_object_token() {
		let tokens: Vec<String> = vec![String::from("{")];
		let result = parse_json(tokens);
		assert_eq!(result, false);

	}

	#[test]
	fn test_end_object_token() {
		let tokens: Vec<String> = vec![String::from("}")];
		let result = parse_json(tokens);
		assert_eq!(result, false);

	}

	#[test]
	fn test_correct_tokens() {
		let tokens: Vec<String> = vec![String::from("{"), String::from("}")];
		let result = parse_json(tokens);
		assert_eq!(result, true);

	}
}