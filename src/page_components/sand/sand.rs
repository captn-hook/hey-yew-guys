
use web_sys::console::info;
use yew::prelude::*;
//use yew::virtual_dom::ListenerKind::onclick;
use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, WebGl2RenderingContext as GL, WebGl2RenderingContext, WebGlUniformLocation};

use log::info;
//use yew::{html, Component, Context, Html, NodeRef};

pub struct Particle {
    pub x: i32,
    pub y: i32,
    pub vx: i32,
    pub vy: i32,
    pub color: String,
}

pub enum ClicType {
    Sand,
    Water,
    Wall,
}

pub struct Click {
    pub x: i32,
    pub y: i32,
    pub clic_type: ClicType,
}

pub enum Msg {
    //this is the message that is sent when the user clicks on the canvas
    Click(Click),
}

pub struct SandGame {
    pub width: i32,
    pub height: i32,
    pub particles: Vec<Particle>,
    pub clicks: Vec<Click>,
}

impl SandGame {
    pub fn update(&mut self) {
        //update the particles
        for particle in self.particles.iter_mut() {
            particle.x += particle.vx;
            particle.y += particle.vy;
            particle.vy += 1;
        }
        //update the clicks
        for click in self.clicks.iter_mut() {
            match click.clic_type {
                ClicType::Sand => {
                    self.particles.push(Particle {
                        x: click.x,
                        y: click.y,
                        vx: 0,
                        vy: 0,
                        color: String::from("yellow"),
                    });
                }
                ClicType::Water => {
                    self.particles.push(Particle {
                        x: click.x,
                        y: click.y,
                        vx: 0,
                        vy: 0,
                        color: String::from("blue"),
                    });
                }
                ClicType::Wall => {
                    self.particles.push(Particle {
                        x: click.x,
                        y: click.y,
                        vx: 0,
                        vy: 0,
                        color: String::from("black"),
                    });
                }
            }
        }
        //clear the clicks
        self.clicks.clear();
    }
}


// Wrap gl in Rc (Arc for multi-threaded) so it can be injected into the render-loop closure.
pub struct SandWindow {
    //node_ref is a reference to the canvas element
    node_ref: NodeRef,
    cb: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
    //shared ref to the sandgame, which stores the states of the particles
    game: Rc<RefCell<SandGame>>,
    refrence_time: Rc<RefCell<f64>>,
}

impl Component for SandWindow {
    //required by yew
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        //create a reference for canvas element
        Self {
            node_ref: NodeRef::default(),
            cb: Rc::new(RefCell::new(None)),
            game: Rc::new(RefCell::new(SandGame {
                width: 800,
                height: 500,
                particles: Vec::new(),
                clicks: Vec::new(),
            })),
            refrence_time: Rc::new(RefCell::new(0.0)),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let onclick = _ctx.link().callback(|e: MouseEvent| {
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
        //create a canvas element from the reference
        html! {
            <canvas class="sandwindow" style="border: 2px solid black;" width="800" height="500" ref={self.node_ref.clone()} onclick={onclick}></canvas>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        //update the game
        self.game.borrow_mut().update();
        
        match msg {
            Msg::Click(click) => {
                info!("click message {} {}", click.x, click.y);
                info!("refrence time {}", self.refrence_time.borrow());
                //set to zero the refrence time
                *self.refrence_time.borrow_mut() = 0.0;
                info!("refrence time {}", self.refrence_time.borrow());
                self.game.borrow_mut().clicks.push(click);
                true
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        // Only start the render loop if it's the first render
        // There's no loop cancellation taking place, so if multiple renders happen,
        // there would be multiple loops running. That doesn't *really* matter here because
        // there's no props update and no SSR is taking place, but it is something to keep in
        // consideration
        if !first_render {
            return;
        }
        info!("rendered");
        
        // Once rendered, store references for the canvas and GL context. These can be used for
        // resizing the rendering area when the window or canvas element are resized, as well as
        // for making GL calls.
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap(); //this can return an Err
        
        let gl = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();

        Self::render_gl(self, gl);
    }
}

impl SandWindow {
    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    fn render_gl(&mut self, gl: WebGl2RenderingContext) {
        // This should log only once -- not once per frame
        
        let mut timestamp = 0.0;
       
        let vert_code = include_str!("./basic.vert");
        let frag_code = include_str!("./basic.frag");

        // This list of vertices will draw two triangles to cover the entire canvas, we will scale it and place it for each particle
        let vertex: [f32; 12] = [
            -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        ];

        let count = self.game.borrow().particles.len() as i32 * 6;
        let size = 2;
        
        let vertices = vec![vertex; self.game.borrow().particles.len()];

        fn cv(v: &Vec<[f32; 12]>) -> [f32; 72] {
            let mut res = [0.0; 72];
            for (i, x) in v.iter().enumerate() {
                for (j, y) in x.iter().enumerate() {
                    res[i * 12 + j] = *y;
                }
            }
            res
        }   

        let vertex_buffer = gl.create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(&cv(&vertices)[..]);

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, vert_code);
        gl.compile_shader(&vert_shader);

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, frag_code);
        gl.compile_shader(&frag_shader);

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));

        // Attach the position vector as an attribute for the GL context.
        let position = gl.get_attrib_location(&shader_program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, size, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);

        // Attach the time as a uniform for the GL context.
        let time = gl.get_uniform_location(&shader_program, "u_time");
        gl.uniform1f(time.as_ref(), timestamp as f32);

        gl.draw_arrays(GL::TRIANGLES, 0, count);

        // Gloo-render's request_animation_frame has this extra closure
        // wrapping logic running every frame, unnecessary cost.
        // Here constructing the wrapped closure just once.

        //self.cb = Rc::new(RefCell::new(None));
        //game loop
        *self.cb.borrow_mut() = Some(Closure::wrap(Box::new({
            let cb = self.cb.clone();
            let refrence_time = self.refrence_time.clone();
            move || {
                // This should repeat every frame
                //timestamp += 20.0;
                *refrence_time.borrow_mut() += 20.0;
                timestamp = refrence_time.borrow().clone();
                //info!("timestamp {}", timestamp);
                gl.uniform1f(time.as_ref(), timestamp as f32);
                gl.draw_arrays(GL::TRIANGLES, 0, count);
                SandWindow::request_animation_frame(cb.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut()>));
        //click handler 
    
        SandWindow::request_animation_frame(self.cb.borrow().as_ref().unwrap());
    }
}