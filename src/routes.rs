use yew::prelude::*;
use yew_router::prelude::*;

use crate::page_components::about::About;
use crate::page_components::home::Home;
use crate::page_components::contact::Contact;
//use crate::page_components::misc::Misc;
use crate::page_components::not_found::NotFound;
use crate::page_components::projects::Projects;
use crate::page_components::sand::sand::SandWindow;

//this controls access to the routes
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/sand")]
    Sand,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/projects")]
    Projects,
    #[at("/*path")]
    Misc { path: String },
}
//switches the html based on the route
pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::NotFound => html! { <NotFound /> },
        Route::Contact => html! { <Contact /> },
        Route::Projects => html! { <Projects /> },
        Route::Sand => html! { <SandWindow /> },
        Route::Misc { path: _ } => html! { <NotFound /> },
    }
}