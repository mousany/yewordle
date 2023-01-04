use yew::prelude::*;
use yew_router::prelude::*;

use crate::views::{switch, AppRoute};

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
          <Switch<AppRoute> render={switch} />
        </BrowserRouter>
    }
}
