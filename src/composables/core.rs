use crate::types::letters::LetterState;

pub fn certificate(guess: &str, answer: &str) -> Vec<LetterState> {
    assert_eq!(guess.len(), answer.len());

    let answers_upper = answer.to_uppercase();
    let guess_upper = guess.to_uppercase();

    let guess_chars: Vec<_> = guess_upper.chars().collect();
    let anwser_chars: Vec<_> = answers_upper.chars().collect();

    let mut result = vec![];
    for i in 0..guess.len() {
        if guess_chars[i] == anwser_chars[i] {
            result.push(LetterState::Correct);
        } else if answers_upper.contains(guess_chars[i]) {
            result.push(LetterState::Present);
        } else {
            result.push(LetterState::Absent);
        }
    }

    result
}
