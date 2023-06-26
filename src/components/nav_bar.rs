
use yew::prelude::*;
use yew_router::prelude::*;

use stylist::yew::styled_component;

use crate::routes::*;

#[styled_component(NavBar)]
pub fn nav_bar() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <div>
            <button {onclick}>{ "Go Home" }</button>
            //create a navbar that expands on hover
            <div class="dropdown">
                <button class="dropbtn">{ "Dropdown" }</button>
                <div class="dropdown-content">
                    <a href="#">{ "Link 1" }</a>
                    <a href="#">{ "Link 2" }</a>
                    <a href="#">{ "Link 3" }</a>
                </div>

                <button class="dropbtn">{ "Dropdown" }</button>
                <div class="dropdown-content">
                    <a href="#">{ "Link B 1" }</a>
                    <a href="#">{ "Link B 2" }</a>
                    <a href="#">{ "Link B 3" }</a>
                    <a href="#">{ "Link B 4" }</a>
                </div>
            </div>
        </div>
    }
}