use yew::prelude::*;
use yew_router::prelude::*;

//use routes
use crate::routes::*;

#[function_component(Sand)]
pub fn sand() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <div>
            <h1>{ "Sand" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}