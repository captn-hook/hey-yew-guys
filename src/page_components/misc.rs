use yew::prelude::*;
use yew_router::prelude::*;

//use routes
use crate::routes::*;

#[derive(Properties, PartialEq, Clone)]
pub struct MiscProps {
    pub path: String,
}

#[function_component(Misc)]
pub fn misc(props: &MiscProps) -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <div>
            <h1>{ "Misc" }</h1>
            <p>{ props.path.clone() }</p>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
