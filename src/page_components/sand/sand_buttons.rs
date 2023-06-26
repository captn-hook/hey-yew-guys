
use yew::prelude::*;
use yew_router::prelude::*;

use stylist::yew::styled_component;

use crate::routes::*;

#[styled_component(SandButtons)]
pub fn sand_buttons() -> Html {
    html! {
        <div class="navbar">
            <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
        </div>
    }
}