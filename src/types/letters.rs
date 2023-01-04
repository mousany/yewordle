#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LetterState {
    Initial,
    Correct,
    Present,
    Absent,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    letter: char,
    state: LetterState,
}

impl Tile {
    pub fn new(letter: char) -> Self {
        Self {
            letter,
            state: LetterState::Initial,
        }
    }
}
