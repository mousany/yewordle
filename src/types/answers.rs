use crate::types::{guesses::STANDARD_GUESSES, words::STANDARD_WORDS};
use chrono::prelude::*;

pub trait Answer {
    fn answer(&self) -> &str;

    fn word_length(&self) -> usize {
        5
    }

    fn trial_bound(&self) -> usize {
        6
    }

    fn is_allowed(&self, guess: &str) -> bool;
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct StandardWordle;

impl StandardWordle {
    pub fn new() -> Self {
        StandardWordle {}
    }
}

impl Answer for StandardWordle {
    fn answer(&self) -> &str {
        let now = DateTime::from(Utc::now());
        let start = DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z").unwrap();
        let diff = now - start;
        let mut days = diff.num_days();
        while days as usize > STANDARD_WORDS.len() {
            days -= STANDARD_WORDS.len() as i64;
        }
        STANDARD_WORDS[days as usize]
        // "comet"
    }

    fn word_length(&self) -> usize {
        5
    }

    fn trial_bound(&self) -> usize {
        6
    }

    fn is_allowed(&self, guess: &str) -> bool {
        assert_eq!(guess.len(), self.word_length());

        let guess_lower = guess.to_lowercase();

        STANDARD_GUESSES
            .iter()
            .any(|guess_word: &&str| guess_word == &guess_lower)
            || STANDARD_WORDS
                .iter()
                .any(|word: &&str| word == &guess_lower)
    }
}
