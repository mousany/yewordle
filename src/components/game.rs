use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component(Game)]
pub fn game() -> Html {
    let currentRowIndex = use_counter(0);

    html! {
      <div id="board">
        { for board.iter().enumerate().map(|(rowIndex, row)| {
          html! {
            <div class="row">
              { for row.iter().enumerate().map(|(colIndex, tile)| {
                html! {
                  <div class="tile">
                    <div class="front">
                      { tile.letter }
                    </div>
                    <div class="back">
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
