use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::{game::Game, keyboard::Keyboard};

use crate::types::{
    answers::{Answer, StandardWordle},
    letters::Tile,
};

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let wordle = StandardWordle::new();
    let mut board = {
        let mut board = Vec::new();
        for _ in 0..wordle.trial_bound() {
            let mut row = Vec::new();
            for _ in 0..wordle.word_length() {
                row.push(Tile::none());
            }
            board.push(row);
        }
        board
    };

    let current_row_index = use_counter(0);
    let game_success = use_bool_toggle(false);

    html! {
        <>
          <header>
            <h1>{ "YEWORDLE" }</h1>
            <a id="source-link"
              href=""
              target="_blank"
            >{ "Source" }</a>
          </header>
          <Game
            board={board}
            current_row_index={*current_row_index as usize}
            game_success={*game_success}
          />
          <Keyboard />
        </>
    }
}
