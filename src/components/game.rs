use yew::prelude::*;

use crate::types::{answers::StandardWordle, letters::LetterState};

#[derive(Clone, Debug, PartialEq)]
struct Tile {
    letter: char,
    state: LetterState,
}

impl Tile {
    fn new(letter: char) -> Self {
        Self {
            letter,
            state: LetterState::Initial,
        }
    }
}

#[function_component(Game)]
pub fn game() -> Html {
    let wordle = StandardWordle::new();
    let board = Vec::new().fill(Tile::new('\0'));

    html! {}
}
