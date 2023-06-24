use yew::prelude::*;
use yew_router::prelude::*;

use stylist::yew::styled_component;

//use routes
use crate::routes::*;

#[styled_component(NotFound)]
pub fn not_found() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        //light red bg oversize text
        <div style="background-color: #ffcccc; font-size: 1.4em; padding: 3em; border-radius: 1em;">
            <h1>{ "404" }</h1>
            <p>{ "Page not found" }</p>
            //centered, bigger, red button
            <button {onclick} style="background-color: #ffc0c0; font-size: .8em; padding: .2em; border-radius: 1em;">{ "Go Home" }</button>
        </div>
    }
}
