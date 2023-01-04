use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::{
    game::Game,
    keyboard::{Keyboard, INIT_LETTER_STATES},
};

use crate::composables::core::certificate;
use crate::types::letters::LetterState;
use crate::types::{
    answers::{Answer, StandardWordle},
    letters::Tile,
};

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let wordle = StandardWordle::new();
    let board = use_list({
        let mut board = Vec::new();
        for _ in 0..wordle.trial_bound() {
            let mut row = Vec::new();
            for _ in 0..wordle.word_length() {
                row.push(Tile::init());
            }
            board.push(row);
        }
        board
    });

    let allow_input = use_bool_toggle(true);

    let current_row_index = use_counter(0);
    let game_success = use_bool_toggle(false);
    let letter_states = use_map(INIT_LETTER_STATES.clone());

    let complete_row = {
        let letter_states = letter_states.clone();
        let board = board.clone();
        let allow_input = allow_input.clone();
        let current_row_index = current_row_index.clone();
        let game_success = game_success.clone();
        let wordle = wordle;
        Callback::from(move |_| {
            if board.current()[*current_row_index as usize]
                .iter()
                .all(|tile| tile.letter.is_some())
            {
                let guess = board.current()[*current_row_index as usize]
                    .iter()
                    .map(|tile| tile.letter.unwrap())
                    .collect::<String>();

                let certificate_result = certificate(&guess, wordle.answer());
                allow_input.set(false);

                log::info!("Certificate result: {:?}", certificate_result);

                if certificate_result
                    .iter()
                    .all(|state| *state == LetterState::Correct)
                {
                    log::info!("Correct!");
                    for (i, state) in certificate_result.iter().enumerate() {
                        letter_states.update(
                            &board.current()[*current_row_index as usize][i]
                                .letter
                                .unwrap(),
                            *state,
                        );
                    }
                    game_success.set(true);
                } else if (*current_row_index as usize) < wordle.trial_bound() - 1 {
                    log::info!("Incorrect!");
                    for (i, state) in certificate_result.iter().enumerate() {
                        board.update(*current_row_index as usize, {
                            let mut row = board.current()[*current_row_index as usize].clone();
                            let mut tile = &mut row[i];
                            tile.state = *state;
                            row
                        });
                    }
                    for (i, state) in certificate_result.iter().enumerate() {
                        letter_states.update(
                            &board.current()[*current_row_index as usize][i]
                                .letter
                                .unwrap(),
                            *state,
                        );
                    }
                    current_row_index.increase();
                    allow_input.set(true);
                } else {
                    log::info!("Game over!");
                }
            } else {
                log::info!("Not enough letters");
            }
        })
    };

    let onkeyup = {
        let board = board.clone();
        let allow_input = allow_input;
        let current_row_index = current_row_index.clone();
        Callback::from(move |key: String| {
            if *allow_input {
                if key == "Enter" {
                    complete_row.emit(());
                } else if key == "Backspace"
                    && board.current()[*current_row_index as usize]
                        .iter()
                        .any(|tile| tile.letter.is_some())
                {
                    let mut row = board.current()[*current_row_index as usize].clone();
                    let mut tile = row
                        .iter_mut()
                        .rev()
                        .find(|tile| tile.letter.is_some())
                        .unwrap();
                    tile.letter = None;
                    board.update(*current_row_index as usize, row);
                } else if key.len() == 1 && key.chars().next().unwrap().is_alphabetic() {
                    let letter = key.chars().next().unwrap();
                    if board.current()[*current_row_index as usize]
                        .iter()
                        .all(|tile| tile.letter.is_some())
                    {
                        return;
                    } else {
                        let mut row = board.current()[*current_row_index as usize].clone();
                        let mut tile = row.iter_mut().find(|tile| tile.letter.is_none()).unwrap();
                        tile.letter = Some(letter);
                        board.update(*current_row_index as usize, row);
                    }
                }
            }
        })
    };

    html! {
      <>
        <header>
          <h1>{ "YEWORDLE" }</h1>
          <a id="source-link"
            href=""
            target="_blank"
          >{ "Source" }</a>
        </header>
        <div>
          {wordle.answer()}
        </div>
        <div>
          <Game
            board={board.current().clone()}
            current_row_index={*current_row_index as usize}
            game_success={*game_success}
          />
        </div>
        <div>
          <Keyboard
            letter_states={letter_states.current().clone()}
            onkeyup={onkeyup}
          />
        </div>
      </>
    }
}
