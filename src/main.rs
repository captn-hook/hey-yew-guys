use yew::prelude::*;
use yew_router::prelude::*;

mod routes;
use routes::*;

mod page_components;

//main app
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}
//starts app
fn main() {
    yew::Renderer::<App>::new().render();
}