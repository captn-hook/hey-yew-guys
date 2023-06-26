
use yew::prelude::*;
use yew_router::prelude::*;

use stylist::yew::styled_component;

use crate::routes::*;

#[styled_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <div class="navbar">
            <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
            <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
            <Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>>
            <Link<Route> to={Route::Projects}>{ "Projects" }</Link<Route>>
            <Link<Route> to={Route::Sand}>{ "Sand" }</Link<Route>>
        </div>
    }
}