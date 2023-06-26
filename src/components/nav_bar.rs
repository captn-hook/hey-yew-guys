
use yew::prelude::*;
use yew_router::prelude::*;

use stylist::yew::styled_component;

use crate::routes::*;

#[styled_component(NavBar)]
pub fn nav_bar() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <div class="navbar">
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}