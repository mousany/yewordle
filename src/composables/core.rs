use crate::types::letters::LetterState;

pub fn certificate(guess: &str, answer: &str) -> Vec<LetterState> {
    assert_eq!(guess.len(), answer.len());

    let answers_upper = answer.to_uppercase();
    let guess_upper = guess.to_uppercase();

    let guess_chars: Vec<_> = guess_upper.chars().collect();
    let mut anwser_letters: Vec<_> = answers_upper.chars().map(Some).collect();

    let mut result = vec![LetterState::Absent; guess.len()];

    // Check for correct letters
    for i in 0..guess.len() {
        if Some(guess_chars[i]) == anwser_letters[i] {
            result[i] = LetterState::Correct;
            anwser_letters[i] = None;
        }
    }

    // Check for present letters
    for i in 0..guess.len() {
        if result[i] == LetterState::Absent && anwser_letters.contains(&Some(guess_chars[i])) {
            result[i] = LetterState::Present;

            let index = anwser_letters
                .iter()
                .position(|x| *x == Some(guess_chars[i]))
                .unwrap();
            anwser_letters[index] = None;
        }
    }

    result
}
