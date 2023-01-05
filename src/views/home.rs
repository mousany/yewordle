use gloo::{
    events::EventListener,
    history::{BrowserHistory, History},
};
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{
    components::{
        game::Game,
        keyboard::{Keyboard, INIT_LETTER_STATES},
        message::Message,
    },
    composables::utils::set_timeout,
    types::letters::match_letter_icon,
};

use crate::composables::core::certificate;
use crate::types::letters::LetterState;
use crate::types::{
    answers::{Answer, StandardWordle},
    letters::Tile,
};

fn get_word_from_url(word_length: usize) -> Option<String> {
    let location = BrowserHistory::new().location();
    let query_str = &location.query_str()[1..];

    if !query_str.is_empty() {
        if let Ok(word) = base64::decode(query_str) {
            let word = String::from_utf8(word).unwrap();
            log::info!("word: {:?}", word);
            if word.is_ascii() && word.len() == word_length {
                return Some(word);
            }
        }
    }

    None
}

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let wordle = StandardWordle::new();
    let wordle_answer = {
        if let Some(word) = get_word_from_url(wordle.word_length()) {
            word
        } else {
            wordle.answer().to_string()
        }
    };

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

    let shake_row_index = use_state(|| Option::<usize>::None);
    let current_row_index = use_counter(0);
    let game_success = use_bool_toggle(false);
    let letter_states = use_map(INIT_LETTER_STATES.clone());

    let hint_message = use_state(|| Option::<String>::None);
    let performance_grid = use_state(|| Option::<String>::None);

    let shake_row = {
        let current_row_index = current_row_index.clone();
        let shake_row_index = shake_row_index.clone();
        Callback::from(move |_: ()| {
            shake_row_index.set(Some(*current_row_index as usize));
            {
                let shake_row_index = shake_row_index.clone();
                set_timeout(
                    move || {
                        shake_row_index.set(None);
                    },
                    1000,
                );
            }
        })
    };

    let show_hint_message = {
        let hint_message = hint_message.clone();
        Callback::from(move |args: (String, i64)| {
            let (message, timeout) = args;
            hint_message.set(Some(message));
            if timeout > 0 {
                let hint_message = hint_message.clone();
                set_timeout(
                    move || {
                        hint_message.set(None);
                    },
                    timeout as u64,
                );
            }
        })
    };

    let generate_performance_grid = {
        let board = board.clone();
        let current_row_index = current_row_index.clone();
        Callback::from(move |_: ()| -> String {
            let mut grid = String::new();
            for row in board.current()[..*current_row_index as usize + 1].iter() {
                for tile in row.iter() {
                    grid.push_str(&match_letter_icon(tile.state));
                }
                grid.push('\n');
            }
            grid
        })
    };

    let dispatch_result = {
        let board = board.clone();
        let letter_states = letter_states.clone();
        let current_row_index = current_row_index.clone();
        Callback::from(move |certificate_result: Vec<LetterState>| {
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
        })
    };

    let complete_row = {
        let board = board.clone();
        let allow_input = allow_input.clone();
        let current_row_index = current_row_index.clone();
        let game_success = game_success.clone();
        let wordle = wordle;
        let wordle_answer = wordle_answer.clone();
        let show_hint_message = show_hint_message.clone();
        let generate_performance_grid = generate_performance_grid;
        let performance_grid = performance_grid.clone();
        Callback::from(move |_| {
            if board.current()[*current_row_index as usize]
                .iter()
                .all(|tile| tile.letter.is_some())
            {
                let guess = board.current()[*current_row_index as usize]
                    .iter()
                    .map(|tile| tile.letter.unwrap())
                    .collect::<String>();

                if !wordle.is_allowed(&guess) && guess != wordle_answer {
                    log::info!("Not allowed: {}", guess);
                    shake_row.emit(());
                    show_hint_message.emit(("Not in word list".to_string(), 1000));
                    return;
                }

                let certificate_result = certificate(&guess, &wordle_answer);
                allow_input.set(false);

                log::info!("Certificate result: {:?}", certificate_result);
                dispatch_result.emit(certificate_result.clone());

                if certificate_result
                    .iter()
                    .all(|state| *state == LetterState::Correct)
                {
                    log::info!("Correct!");
                    {
                        let game_success = game_success.clone();
                        let show_hint_message = show_hint_message.clone();
                        let generate_performance_grid = generate_performance_grid.clone();
                        let current_row_index = current_row_index.clone();
                        let performance_grid = performance_grid.clone();
                        set_timeout(
                            move || {
                                performance_grid.set(Some(generate_performance_grid.emit(())));
                                show_hint_message.emit((
                                    wordle.gamer_level(*current_row_index as usize).to_string(),
                                    -1,
                                ));
                                game_success.set(true);
                            },
                            1600,
                        )
                    }
                } else if (*current_row_index as usize) < wordle.trial_bound() - 1 {
                    log::info!("Incorrect!");
                    current_row_index.increase();
                    {
                        let allow_input = allow_input.clone();
                        set_timeout(
                            move || {
                                allow_input.set(true);
                            },
                            1600,
                        );
                    }
                } else {
                    log::info!("Game over!");
                    show_hint_message.emit((wordle_answer.to_uppercase(), -1));
                }
            } else {
                shake_row.emit(());
                log::info!("Not enough letters");
                show_hint_message.emit(("Not enough letters".to_string(), 1000));
            }
        })
    };

    let onkeyup = {
        let board = board.clone();
        let allow_input = allow_input.clone();
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
                    let letter = key.to_uppercase().chars().next().unwrap();
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

    let keyboardup_listener = {
        if let Some(window) = window() {
            let onkeyup = onkeyup.clone();
            Some(EventListener::new(&window, "keyup", move |event| {
                let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
                // log::info!("Key up at {:?}", event.key());
                onkeyup.emit(event.key());
            }))
        } else {
            allow_input.set(false);
            show_hint_message.emit(("Failed to mount on window".to_string(), -1));
            None
        }
    };

    use_unmount(move || {
        if let Some(listener) = keyboardup_listener {
            listener.forget();
        }
    });

    html! {
      <>
        <Message
          message={(*hint_message).clone()}
          grid={(*performance_grid).clone()}
        />
        <header>
          <h1>{ "YEWORDLE" }</h1>
          <a id="source-link"
            href="https://github.com/yanglinshu/yewordle"
            target="_blank"
          >{ "Source" }</a>
        </header>
        <div>
          {wordle_answer}
        </div>
        <div>
          <Game
            board={board.current().clone()}
            current_row_index={*current_row_index as usize}
            shake_row_index={*shake_row_index}
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
