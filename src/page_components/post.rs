use yew::prelude::*;
use yew_router::prelude::*;

//use routes
use crate::routes::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PostProps {
    pub id: String,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <div>
            <h1>{ "Post" }</h1>
            <p>{ props.id.clone() }</p>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
