use yew::prelude::*;
use yew_router::prelude::*;

use crate::page_components::secure::Secure;
use crate::page_components::home::Home;
//use crate::page_components::post::PostProps;
use crate::page_components::post::Post;
//use crate::page_components::misc::MiscProps;
use crate::page_components::misc::Misc;


//this controls access to the routes
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/post/:id")]
    Post { id: String },
    #[at("/*path")]
    Misc { path: String },
}
//switches the html based on the route
pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Secure => html! { <Secure /> },
        Route::NotFound => html! { <p>{ "404" }</p> },
        Route::Post { id } => html! { <Post id={id} /> },
        Route::Misc { path } => html! { <Misc path={path} /> },
    }
}