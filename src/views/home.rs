use crate::types::answers::{Answer, StandardWordle};
use yew::prelude::*;
use yew_hooks::prelude::*;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let wordle = StandardWordle::new();
    html! {
        <div class="app">
          <div>
            { wordle.answer() }
          </div>
        </div>
    }
}
