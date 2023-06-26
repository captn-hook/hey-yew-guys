use yew::prelude::*;
use yew_router::prelude::*;
use stylist::yew::styled_component;

use crate::theme::foreground;
use crate::routes::*;

// #[function_component(SecureBT)]
// pub fn secure_button() -> Html {
//     let navigator = use_navigator().unwrap();
//     let onclick = Callback::from(move |_| navigator.push(&Route::Secure));

//     html! {
//         <button {onclick}>{ "Go Secure" }</button>
//     }
// }

// #[function_component(NotFoundBT)]
// pub fn not_found_button() -> Html {
//     let navigator = use_navigator().unwrap();
//     let onclick = Callback::from(move |_| navigator.push(&Route::NotFound));

//     html! {
//         <button {onclick}>{ "Go 404" }</button>
//     }
// }

// #[function_component(PostSelect)]
// pub fn post_select() -> Html {
//     let navigator = use_navigator().unwrap();
//     let onclick = Callback::from(move |_| {
//         let window = web_sys::window().unwrap();
//         let document = window.document().unwrap();
//         let post_id = document.get_element_by_id("post_id").unwrap();
//         let post_id = post_id.dyn_into::<web_sys::HtmlInputElement>().unwrap();
//         navigator.push(&Route::Post { id: post_id.value() });
//     });

//     html! {
//         <div>
//             <input id="post_id" type="text" placeholder="Post ID" />
//             <button {onclick}>{ "Go Post" }</button>
//         </div>
//     }
// }

// #[function_component(PathSelect)]
// pub fn path_select() -> Html {
//     let navigator = use_navigator().unwrap();
//     let onclick = Callback::from(move |_| {
//         let window = web_sys::window().unwrap();
//         let document = window.document().unwrap();
//         let path = document.get_element_by_id("path").unwrap();
//         let path = path.dyn_into::<web_sys::HtmlInputElement>().unwrap();
//         navigator.push(&Route::Misc { path: path.value() });
//     });

//     html! {
//         <div>
//             <input id="path" type="text" placeholder="Path" />
//             <button {onclick}>{ "Go Path" }</button>
//         </div>
//     }
// }

#[function_component(AboutBT)]
pub fn about_button() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::About));

    html! {
        <button {onclick}>{ "Go About" }</button>
    }
}

#[function_component(ProjectsBT)]
pub fn projects_button() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Projects));

    html! {
        <button {onclick}>{ "Go Projects" }</button>
    }
}

#[function_component(SandBT)]
pub fn sand_button() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Sand));

    html! {
        <button {onclick}>{ "Go Sand" }</button>
    }
}

#[function_component(ContactBT)]
pub fn contact_button() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Contact));

    html! {
        <button {onclick}>{ "Go Contact" }</button>
    }
}

#[styled_component(Home)]
pub fn home() -> Html {
    html! {
        //round corners
        <div class={foreground()} style="border-radius: 25px;">
            <h1>{ "Home" }</h1>
            // < SecureBT />
            // < NotFoundBT />
            // < PostSelect />
            // < PathSelect />
            < AboutBT />
            < ContactBT />
            < ProjectsBT />
            < SandBT />
        </div>
    }
}