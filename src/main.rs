use yew::prelude::*;
use yew_router::prelude::*;
use stylist::yew::styled_component;
use stylist::yew::Global;

mod routes;
use routes::*;

mod page_components;

mod components;
use crate::components::nav_bar::NavBar;

mod theme;
use theme::light_theme;

use log::info;

//main app
#[styled_component(App)]
fn app() -> Html {
    let theme = light_theme();
    html! {
        //all elements must have a single root element in yew::html()!
        <>
            // Global Styles can be applied with <Global /> component.
            <Global css={theme.css}/>
         
            <BrowserRouter>
                <NavBar />
                <div class="page">
                    <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
                </div>
            </BrowserRouter>
        </>
    }
}
//starts app
fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    info!("Starting app");
    yew::Renderer::<App>::new().render();
}
