use yew::prelude::*;
use web_sys::{window, HtmlCanvasElement, WebGlRenderingContext as GL, MouseEvent};
use wasm_logger;
use log;

#[function_component(Simulation)]
fn simulation() -> Html {
    wasm_logger::init(wasm_logger::Config::default());  
    //simulation is a canvas element with event listeners for mouse
    let onclick = Callback::from(|e: MouseEvent| {
        let x = e.client_x();
        let y = e.client_y();
        log::info!("Mouse clicked at ({}, {})", x, y);
    });
    html! {
        <canvas id="simulation" width="500" height="500" onclick={Callback::from(onclick)}></canvas>
    }
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <Simulation />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}