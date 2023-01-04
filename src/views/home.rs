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
        for _ in 0..6 {
            let mut row = Vec::new();
            for _ in 0..wordle.length() {
                row.push(Tile::new(' '));
            }
            board.push(row);
        }
        board
    };

    let currentRowIndex = use_counter(0);

    html! {
        <>
          <header>
            <h1>{ "YEWORDLE" }</h1>
            <a id="source-link"
              href=""
              target="_blank"
            >{ "Source" }</a>
          </header>
          <Game />
          <Keyboard />
        </>
    }
}
