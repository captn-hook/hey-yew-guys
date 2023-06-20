use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;
use web_sys::{ window, HtmlCanvasElement, WebGlRenderingContext as GL, MouseEvent };
use yew::{ html, Component, Context, Html, NodeRef };
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

//grid size is the number of particles in each dimension
const GRID_SIZE: usize = 100;

//imp std::marker::Copy for Particle {}
#[derive(Copy, Clone)]
pub struct Particle {
    size: u32,
    id: u32,
    position: (u32, u32),
    force: (f32, f32),
    mass: f32,
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

//particles are event driven, and get their pos from the sim grid
impl Particle {
    //new particle takes a counter and a size to determine id on grid
    //and some physics constants
    pub fn new(size: u32) -> Self {
        Particle {
            size: size,
            position: (0, 0),
            id: 0,
            force: (0.0, 0.0),
            mass: 1.0, //negative mass is upward force
            //eventually i want to subclass particle into different types with interactions
        }
    }

    pub fn copy(part: &Particle, new_pos: (u32, u32)) -> Self {
        Particle {
            size: part.size,
            position: new_pos,
            id: part.id,
            force: part.force,
            mass: part.mass,
        }
    }

    // :[ 
    pub fn init_part(&mut self, counter: u32) {
        self.id = counter;
        self.position = ((self.id % self.size) as u32, (self.id / self.size) as u32);
    }
}
   
pub struct Simulation<'a> {
    //simulation defines a size and some physics constants
    //particle refrences are stored in a grid
    particles: [[&'a mut Particle; GRID_SIZE]; GRID_SIZE],
    gravity: f32,
    time_step: u32,
    loopx: bool,
    loopy: bool,
}

impl Simulation<'_> {
    pub fn new(gravity: f32, time_step: u32, loopx: bool, loopy: bool) -> Self {
        Simulation {
            particles: Simulation::init_particles(),
            gravity: gravity,
            time_step: time_step,
            loopx: loopx,
            loopy: loopy,
        }
    }

    pub fn empty_grid() -> [[&'static mut Particle; GRID_SIZE]; GRID_SIZE] {
        let grid = [[None; GRID_SIZE]; GRID_SIZE];

        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                grid[i][j] = &mut Particle::new(GRID_SIZE as u32);
            }
        }
        grid
    }
   
    pub fn init_particles() -> [[&'static mut Particle; GRID_SIZE]; GRID_SIZE] {
        let mut counter = 0;
        let mut vec_particles: Vec<Vec<Particle>> = Vec::new();
        let mut particles = Simulation::empty_grid();
        
        let mut vec_particles: Vec<Vec<Particle>> = Vec::new();
        
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let mut part = Particle::new(GRID_SIZE as u32);
                part.init_part(counter);
                vec_particles[i].push(part);
                counter += 1;
            }
        }

        //convert vec to array
        //????

        particles
    }

    pub fn get_positions(&self) -> Vec<f32> {
        let mut positions: Vec<f32> = Vec::new();
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                positions.push(self.particles[i][j].position.0 as f32);
                positions.push(self.particles[i][j].position.1 as f32);
                positions.push(0.0);
            }
        }
        positions
    }
}

pub struct App {
    node_ref: NodeRef,
    gl: Option<GL>,
    canvas: Option<HtmlCanvasElement>,
    last_click: Option<(f32, f32)>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            node_ref: NodeRef::default(),
            gl: None,
            canvas: None,
            last_click: None,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let onclick = Callback::from(|e: MouseEvent| {
            let x = e.client_x();
            let y = e.client_y();
        });

        html! {
            <canvas ref={self.node_ref.clone()} id="simulation" width="900" height="600" onclick={onclick}></canvas>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
        let gl: GL = canvas.get_context("webgl").unwrap().unwrap().dyn_into().unwrap();
        Self::render_gl(self, gl);
    }
}

fn setup_gl(gl: &GL) {
    let vert_code = include_str!("./basic.vert");
    //let frag_code = include_str!("./basic.frag");

    let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
    gl.shader_source(&vert_shader, vert_code);
    gl.compile_shader(&vert_shader);

    //let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
    //gl.shader_source(&frag_shader, frag_code);
    //gl.compile_shader(&frag_shader);

    let shader_program = gl.create_program().unwrap();
    gl.attach_shader(&shader_program, &vert_shader);
    //gl.attach_shader(&shader_program, &frag_shader);
    gl.link_program(&shader_program);
    gl.use_program(Some(&shader_program));

    let position_attrib = gl.get_attrib_location(&shader_program, "position") as u32;
    gl.vertex_attrib_pointer_with_i32(position_attrib, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(position_attrib);
}

impl App {
    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window().unwrap().request_animation_frame(f.as_ref().unchecked_ref()).unwrap();
    }

    fn render_gl(&mut self, gl: GL) {
        let mut timestamp = 0.0;
        // create a simulation that can be passed to the render function
        let mut simulation = Simulation::new(0.0, 0, false, false);
        // get the position list from the simulation
        let positions: Vec<f32> = simulation.get_positions();
        log::info!("positions: {:?}", positions);

        let vert_code = include_str!("./basic.vert");
        let frag_code = include_str!("./basic.frag");

        let vertices = positions;

        let vertex_buffer = gl.create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(vertices.as_slice());

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
        gl.vertex_attrib_pointer_with_i32(position, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);

        // Attach the time as a uniform for the GL context.
        let time = gl.get_uniform_location(&shader_program, "u_time");
        gl.uniform1f(time.as_ref(), timestamp as f32);

        gl.draw_arrays(GL::POINTS, 0, 6);

        // Gloo-render's request_animation_frame has this extra closure
        // wrapping logic running every frame, unnecessary cost.
        // Here constructing the wrapped closure just once.

        let cb = Rc::new(RefCell::new(None));

        *cb.borrow_mut() = Some(
            Closure::wrap(
                Box::new({
                    let cb = cb.clone();
                    move || {
                        // This should repeat every frame
                        timestamp += 20.0;
                        gl.uniform1f(time.as_ref(), timestamp as f32);

                        // Update the simulation
                        // Get the new positions
                        let positions = simulation.get_positions();
                        // Update the vertex buffer
                        let verts = js_sys::Float32Array::from(positions.as_slice());
                        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
                        gl.buffer_data_with_array_buffer_view(
                            GL::ARRAY_BUFFER,
                            &verts,
                            GL::STATIC_DRAW
                        );

                        // Draw the new frame
                        gl.draw_arrays(GL::POINTS, 0, 6);
                        App::request_animation_frame(cb.borrow().as_ref().unwrap());
                    }
                }) as Box<dyn FnMut()>
            )
        );

        App::request_animation_frame(cb.borrow().as_ref().unwrap());
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
