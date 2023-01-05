#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LetterState {
    Initial,
    Correct,
    Present,
    Absent,
}

pub fn match_letter_state(letter_state: LetterState) -> String {
    match letter_state {
        LetterState::Initial => "".to_string(),
        LetterState::Correct => "correct".to_string(),
        LetterState::Present => "present".to_string(),
        LetterState::Absent => "absent".to_string(),
    }
}

pub fn match_letter_icon(letter_state: LetterState) -> String {
    match letter_state {
        LetterState::Initial => "".to_string(),
        LetterState::Correct => "🟩".to_string(),
        LetterState::Present => "🟨".to_string(),
        LetterState::Absent => "⬜".to_string(),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    pub letter: Option<char>,
    pub state: LetterState,
}

impl Tile {
    pub fn new(letter: char) -> Self {
        Self {
            letter: Some(letter),
            state: LetterState::Initial,
        }
    }

    pub fn init() -> Self {
        Self {
            letter: None,
            state: LetterState::Initial,
        }
    }
}
