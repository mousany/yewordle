use std::{collections::HashMap, vec};

use lazy_static::lazy_static;
use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

use crate::types::letters::{match_letter_state, LetterState};

lazy_static! {
    pub static ref INIT_LETTER_STATES: HashMap<char, LetterState> = {
        let mut map = HashMap::new();
        for letter in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            map.insert(letter, LetterState::Initial);
        }
        map
    };
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct KeyboardProps {
    #[prop_or(HashMap::new())]
    pub letter_states: HashMap<char, LetterState>,
    pub onkeyup: Callback<String>,
}

lazy_static! {
    static ref KEYS: Vec<Vec<&'static str>> = [
        ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"].to_vec(),
        ["A", "S", "D", "F", "G", "H", "J", "K", "L"].to_vec(),
        ["Enter", "Z", "X", "C", "V", "B", "N", "M", "Backspace"].to_vec()
    ]
    .to_vec();
}

#[styled_component(Keyboard)]
pub fn keyboard(props: &KeyboardProps) -> Html {
    let KeyboardProps {
        letter_states,
        onkeyup,
    } = props;

    let keyboard_style = use_style!(
        r#"
          margin: 30px 8px 0;
          user-select: none;

          button {
            font-family: inherit;
            font-weight: bold;
            border: 0;
            padding: 0;
            margin: 0 6px 0 0;
            height: 58px;
            border-radius: 4px;
            cursor: pointer;
            user-select: none;
            background-color: #d3d6da;
            color: #1a1a1b;
            flex: 1;
            display: flex;
            justify-content: center;
            align-items: center;
            text-transform: uppercase;
            -webkit-tap-highlight-color: rgba(0, 0, 0, 0.3);
            transition: all 0.2s 1.5s;
          }

          button:last-of-type {
            margin: 0;
          }

          button .big {
            flex: 1.5;
          }
      "#
    );

    let row_style = use_style!(
        r#"
          display: flex;
          width: 100%;
          margin: 0 auto 8px;
          touch-action: manipulation;
      "#
    );

    let spacer_style = use_style!(
        r#"
          flex: 0.5;
      "#
    );

    html! {
      <div class={keyboard_style.clone()} id="keyboard">
        { for KEYS.iter().enumerate().map(|(row_index, row)| {
          html! {
            <div class={row_style.clone()}>
              if row_index == 1 {
                <div class={spacer_style.clone()}></div>
              }
              { for row.iter().enumerate().map(|(col_index, key)| {
                html! {
                  <button
                    onclick={
                      let onkeyup = onkeyup.clone();
                      Callback::from(move |_| {
                        onkeyup.emit(String::from(*key));
                      })
                    }
                    style={
                      if key.len() > 1 {
                        "flex: 1.5;"
                      } else {
                        ""
                      }
                    }
                    class={
                      if key.len() == 1 {
                        vec![match_letter_state(
                          *letter_states.get(&key.chars().next().unwrap()).unwrap()
                        )]
                      } else {
                        vec![]
                      }
                    }
                  >
                    if *key != "Backspace" {
                      <span>
                        { key }
                      </span>
                    } else {
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        height="24"
                        viewBox="0 0 24 24"
                        width="24"
                      >
                        <path
                          fill="currentColor"
                          d="M22 3H7c-.69 0-1.23.35-1.59.88L0 12l5.41 8.11c.36.53.9.89 1.59.89h15c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H7.07L2.4 12l4.66-7H22v14zm-11.59-2L14 13.41 17.59 17 19 15.59 15.41 12 19 8.41 17.59 7 14 10.59 10.41 7 9 8.41 12.59 12 9 15.59z"
                        ></path>
                      </svg>
                    }
                  </button>
                }
              }) }
              if row_index == 1 {
                <div class={spacer_style.clone()}></div>
              }
            </div>
          }
        }) }
      </div>
    }
}
