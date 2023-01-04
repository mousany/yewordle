use stylist::{style, yew::use_style, Style};
use yew::prelude::*;

use crate::types::letters::{match_letter_state, LetterState, Tile};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct GameProps {
    pub board: Vec<Vec<Tile>>,
    pub shake_row_index: Option<usize>,
    #[prop_or(0)]
    pub current_row_index: usize,
    #[prop_or(false)]
    pub game_success: bool,
}

#[function_component(Game)]
pub fn game(props: &GameProps) -> Html {
    let GameProps {
        board,
        shake_row_index,
        current_row_index,
        game_success,
    } = props;

    let board_style = use_style!(
        r#"
        display: grid;
        grid-template-rows: repeat(6, 1fr);
        grid-gap: 5px;
        padding: 10px;
        box-sizing: border-box;
        --height: min(420px, calc(var(--vh, 100vh) - 310px));
        height: var(--height);
        width: min(350px, calc(var(--height) / 6 * 5));
        margin: 0px auto;
      "#
    );

    let board_row_style = use_style!(
        r#"
        display: grid;
        grid-template-columns: repeat(5, 1fr);
        grid-gap: 5px;        
      "#
    );

    let board_row_shake_style = use_style!(
        r#"
      animation: shake 0.5s;      
      @keyframes shake {
        0% {
          transform: translate(1px);
        }
        10% {
          transform: translate(-2px);
        }
        20% {
          transform: translate(2px);
        }
        30% {
          transform: translate(-2px);
        }
        40% {
          transform: translate(2px);
        }
        50% {
          transform: translate(-2px);
        }
        60% {
          transform: translate(2px);
        }
        70% {
          transform: translate(-2px);
        }
        80% {
          transform: translate(2px);
        }
        90% {
          transform: translate(-2px);
        }
        100% {
          transform: translate(1px);
        }
      }
      "#
    );

    let board_tile_style = use_style!(
        r#"
      width: 100%;
      font-size: 2rem;
      line-height: 2rem;
      font-weight: bold;
      vertical-align: middle;
      text-transform: uppercase;
      user-select: none;
      position: relative;
      
      & .filled {
        animation: zoom 0.2s;
      }

      & .front, .back {
        box-sizing: border-box;
        display: inline-flex;
        justify-content: center;
        align-items: center;
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        transition: transform 0.6s;
        backface-visibility: hidden;
        -webkit-backface-visibility: hidden;
      }

      & .front {
        border: 2px solid #d3d6da;
      }

      & .filled .front {
        border-color: #999;
      }

      & .back {
        transform: rotateX(180deg);
      }

      & .revealed .front {
        transform: rotateX(180deg);
      }

      & .revealed .back {
        transform: rotateX(0deg);
      }

      @keyframes zoom {
        0% {
          transform: scale(1.1);
        }
        100% {
          transform: scale(1);
        }
      }

      @media (max-height: 680px) {
        & {
          font-size: 3vh;
        }
      }
      "#
    );

    let board_tile_jump_style = style!(
        r#"
        & .back {
          animation: jump 0.5s;
        }
        @keyframes jump {
          0% {
            transform: translateY(0px);
          }
          20% {
            transform: translateY(5px);
          }
          60% {
            transform: translateY(-25px);
          }
          90% {
            transform: translateY(3px);
          }
          100% {
            transform: translateY(0px);
          }
        }
      "#
    )
    .unwrap();

    html! {
      <div id="board" class={board_style.clone()}>
        { for board.iter().enumerate().map(|(row_index, row)| {
          html! {
            <div class={
              [vec![board_row_style.clone()], {
                if Some(row_index) == *shake_row_index {
                  vec![board_row_shake_style.clone()]
                } else {
                  vec![]
                }
              }, {
                if *game_success && row_index == *current_row_index {
                  vec![board_tile_jump_style.clone()]
                } else {
                  vec![]
                }
              }].concat()
            }>
              { for row.iter().enumerate().map(|(col_index, tile)| {
                html! {
                  <div class={
                    vec![
                      board_tile_style.get_class_name().to_string(),
                      {
                        match tile.letter {
                          Some(_) => "filled".to_string(),
                          None => "".to_string(),
                        }
                      },
                      {
                        match tile.state {
                          LetterState::Initial => "".to_string(),
                          _ => "revealed".to_string(),
                        }
                      }
                    ]
                  }>
                    <div class="front"
                      style={
                        format!("transition-delay: {}ms", col_index * 300)
                      }
                    >
                      { tile.letter }
                    </div>
                    <div
                      class={
                        vec![
                          "back".to_string(), match_letter_state(tile.state)
                        ]
                      }
                      style={
                        format!("transition-delay: {}ms; animation-delay: {}ms;", col_index * 300, col_index * 100)
                      }
                    >
                      { tile.letter }
                    </div>
                  </div>
                }
              }) }
            </div>
          }
        }) }
      </div>
    }
}
