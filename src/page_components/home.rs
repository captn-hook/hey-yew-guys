use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::JsCast;

//use routes
use crate::routes::*;

#[function_component(SecureBT)]
pub fn secure_button() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Secure));

    html! {
        <button {onclick}>{ "Go Secure" }</button>
    }
}

#[function_component(NotFoundBT)]
pub fn not_found_button() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::NotFound));

    html! {
        <button {onclick}>{ "Go 404" }</button>
    }
}

#[function_component(PostSelect)]
pub fn post_select() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let post_id = document.get_element_by_id("post_id").unwrap();
        let post_id = post_id.dyn_into::<web_sys::HtmlInputElement>().unwrap();
        navigator.push(&Route::Post { id: post_id.value() });
    });

    html! {
        <div>
            <input id="post_id" type="text" placeholder="Post ID" />
            <button {onclick}>{ "Go Post" }</button>
        </div>
    }
}

#[function_component(PathSelect)]
pub fn path_select() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let path = document.get_element_by_id("path").unwrap();
        let path = path.dyn_into::<web_sys::HtmlInputElement>().unwrap();
        navigator.push(&Route::Misc { path: path.value() });
    });

    html! {
        <div>
            <input id="path" type="text" placeholder="Path" />
            <button {onclick}>{ "Go Path" }</button>
        </div>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();
    
    html! {
        <div>
            <h1>{ "Home" }</h1>
            < SecureBT />
            < NotFoundBT />
            < PostSelect />
            < PathSelect />
        </div>
    }
}