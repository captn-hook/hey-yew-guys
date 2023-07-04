
#[allow(unused)]
use web_sys::console::info;
use yew::context;
use yew::prelude::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::cell::RefCell;
use std::rc::Rc;

use super::sand_buttons::*;
use super::sandgame::*;
use super::gl::*;


use web_sys::{window, HtmlCanvasElement, WebGl2RenderingContext as GL, WebGl2RenderingContext, WebGlUniformLocation};

// Wrap gl in Rc (Arc for multi-threaded) so it can be injected into the render-loop closure.
pub struct SandWindow {
    //node_ref is a reference to the canvas element
    node_ref: NodeRef,
    glc: GlMgr,
    game: SandGame,
    buttons: SandButtons,
}

impl Component for SandWindow {
    //required by yew
    type Message = Msg; //this contains click msg and sandbutton msgs
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let node_ref = NodeRef::default();
        //create a reference for canvas element
        let canvas = node_ref.cast::<HtmlCanvasElement>().unwrap(); //this can return an Err
        
        let glc_context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();

        let cb = Rc::new(RefCell::new(None));
        let refrence_time = Rc::new(RefCell::new(0.0));
        Self {
            node_ref,
            glc: GlMgr::new(glc_context, cb.clone(), refrence_time.clone()),
            game: SandGame {
                width: 0,
                height: 0,
                particles: Vec::new(),
                clicks: Vec::new(),
            },
            buttons: SandButtons {
                buttons: vec![
                    Buttons::Rocket,
                    Buttons::Moon,
                    Buttons::Earth,
                    Buttons::Sun,
                    Buttons::Star,
                ],
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|e: MouseEvent| {
            //info!("click lsitener");
            //get the position of the click
            let x = e.client_x();
            let y = e.client_y();
            //get the type of click
            let clic_type = ClicType::Sand;
            //create a click object
            let click = Click {
                x: x,
                y: y,
                clic_type: clic_type,
            };
            //send the click message
            Msg::Click(click)
        });

        html! {
            <div class="sandwindow">
                <SandButtons />
                <canvas class="viewer" style="border: 2px solid black;" width="800" height="500" ref={self.node_ref.clone()} onclick={onclick}></canvas>
            </div>
        }
    }
        
    }