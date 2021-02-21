// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Tetsy.

// Tetsy is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tetsy is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tetsy.  If not, see <http://www.gnu.org/licenses/>.

//! Tetsy Brain Wallet Generator.

#![warn(missing_docs)]

use std::fmt;
use std::collections::HashSet;
use rand::{rngs::OsRng, seq::SliceRandom};

/// The list of dictionary words.
// the wordlist JSON also happens to be valid Rust syntax for an array constant.
pub const WORDS: &'static [&'static str] = &include!("../res/wordlist.json");

/// Generate a string which is a random phrase of a number of lowercase words.
///
/// `words` is the number of words, chosen from a dictionary of 7,530. An value of
/// 12 gives 155 bits of entropy (almost saturating address space); 20 gives 258 bits
/// which is enough to saturate 32-byte key space
pub fn random_phrase(no_of_words: usize) -> String {
	let mut rng = OsRng;
	(0..no_of_words).map(|_| WORDS.choose(&mut rng).unwrap()).fold(String::new(), |mut acc, word| {
		acc.push_str(" ");
		acc.push_str(word);
		acc
	}).trim_start().to_owned()
}

/// Phrase Validation Error
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
	/// Phrase is shorter than it was expected.
	PhraseTooShort(usize),
	/// Phrase contains a word that doesn't come from our dictionary.
	WordNotFromDictionary(String),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::PhraseTooShort(len) => writeln!(fmt, "The phrase is too short ({})", len),
            Error::WordNotFromDictionary(ref word) => writeln!(fmt, "The word '{}' does not come from the dictionary.", word),
        }
    }
}

/// Validates given phrase and checks if:
/// 1. All the words are coming from the dictionary.
/// 2. There are at least `expected_no_of_words` in the phrase.
pub fn validate_phrase(phrase: &str, expected_no_of_words: usize) -> Result<(), Error> {
	lazy_static::lazy_static! {
		static ref WORD_SET: HashSet<&'static str> = WORDS.iter().cloned().collect();
	}

	let mut len = 0;
	for word in phrase.split_whitespace() {
		len += 1;
		if !WORD_SET.contains(word) {
			return Err(Error::WordNotFromDictionary(word.into()));
		}
	}

	if len < expected_no_of_words {
		return Err(Error::PhraseTooShort(len));
	}

	return Ok(());
}

#[cfg(test)]
mod tests {
	use super::{validate_phrase, random_phrase, Error};

	#[test]
	fn should_produce_right_number_of_words() {
		let p = random_phrase(10);
		assert_eq!(p.split(" ").count(), 10);
	}

	#[test]
	fn should_not_include_carriage_return() {
		let p = random_phrase(10);
		assert!(!p.contains('\r'), "Carriage return should be trimmed.");
	}

	#[test]
	fn should_validate_the_phrase() {
		let p = random_phrase(10);

		assert_eq!(validate_phrase(&p, 10), Ok(()));
		assert_eq!(validate_phrase(&p, 12), Err(Error::PhraseTooShort(10)));
		assert_eq!(validate_phrase("xxx", 0), Err(Error::WordNotFromDictionary("xxx".into())));
	}
}
