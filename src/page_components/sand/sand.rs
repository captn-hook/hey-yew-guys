
#[allow(unused)]
use web_sys::console::info;
use yew::prelude::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::sand_buttons::*;
use super::sandgame::*;
use super::gl::*;

// Wrap gl in Rc (Arc for multi-threaded) so it can be injected into the render-loop closure.
pub struct SandWindow {
    //node_ref is a reference to the canvas element
    node_ref: NodeRef,
    glc: Gl_Mgr,
    game: SandGame,
    buttons: SandButtons,
}

impl Component for SandWindow {
    //required by yew
    type Message = Msg; //this contains click msg and sandbutton msgs
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        //create a reference for canvas element
        Self {
            node_ref: NodeRef::default(),
            glc: Gl_Mgr::new(),
            game: SandGame::new(800, 600),
            buttons: SandButtons::new(),
        }
    }

    fn view(&self,


