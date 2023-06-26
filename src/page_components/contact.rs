use yew::prelude::*;
use yew_router::prelude::*;

//use routes
use crate::routes::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <div>
            <h1>{ "Contact" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}