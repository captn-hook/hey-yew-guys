use yew::prelude::*;
use yew_router::prelude::*;
use stylist::yew::styled_component;
use stylist::yew::Global;

mod routes;
use routes::*;

mod page_components;

mod theme;
use theme::light_theme;

//main app
#[styled_component(App)]
fn app() -> Html {
    let theme = light_theme();
    html! {
        // Global Styles can be applied with <Global /> component.
        <BrowserRouter>
            <Global css={theme.css}/>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
          
        </BrowserRouter>
    }
}
//starts app
fn main() {
    yew::Renderer::<App>::new().render();
}
