use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;
use web_sys::{window, HtmlCanvasElement, WebGlRenderingContext as GL, MouseEvent};
use yew::{html, Component, Context, Html, NodeRef};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

#[derive(Debug)]
pub struct Particle {
    position: (f32, f32),
    velocity: (f32, f32),
    mass: f32,
}

fn psuedo_rand() -> f32 {
    //get a digit of pi from the current time
    let pi = std::f32::consts::PI;
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let digit = (time % 10) as f32;
    //return the digit as a float between 0 and 1
    digit / 10.0
}

impl Particle {
    //default particle to x,y, 0,0, 0,0, 1
    pub fn new(x: f32, y: f32) -> Self {
        Particle {
            position: (x, y),
            velocity: (0.0, 0.0),
            mass: 1.0,
        }
    }

    //custom particle
    pub fn create(x: f32, y: f32, vx: f32, vy: f32, m: f32) -> Self {
        Particle {
            position: (x, y),
            velocity: (vx, vy),
            mass: m,
        }
    }

    //fun spawn
    pub fn spawn(x: f32, y: f32) -> Self {
        let vx = psuedo_rand() * 2.0 - 1.0;
        let vy = psuedo_rand() * 2.0 - 1.0;
        Particle {
            position: (x, y),
            velocity: (vx, vy),
            mass: 1.0,
        }
    }

    //format particle
    pub fn format(&self) -> String {
        format!(
            "Particle: position: ({}, {}), velocity: ({}, {}), mass: {}",
            self.position.0, self.position.1, self.velocity.0, self.velocity.1, self.mass
        )
    }
}

pub struct Simulation {
    size: (u32, u32),
    particles: Vec<Particle>,
    gravity: f32,
}

impl Simulation {
    pub fn new(size: (u32, u32)) -> Self {
        let particles = vec![
            Particle::create(0.0, 1.0, 0.0, -1.0, 1.0),
            Particle::create(0.0, -1.0, 0.0, 1.0, 1.0),
            Particle::create(1.0, 0.0, -1.0, 0.0, 1.0),            
        ];
        Simulation {
            size,
            particles,
            gravity: -0.01,
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn update(&mut self, delta_time: f32) {
        for particle in self.particles.iter_mut() {
            particle.position.0 += particle.velocity.0 * delta_time;
            particle.position.1 += particle.velocity.1 * delta_time;
            particle.velocity.0 *= 0.99;
            particle.velocity.1 *= 0.99;

            particle.velocity.1 += self.gravity * delta_time;

            if particle.position.0 < -1.0 {
                particle.position.0 = -1.0;
                particle.velocity.0 *= -1.0;
            } else if particle.position.0 > 1.0 {
                particle.position.0 = 1.0;
                particle.velocity.0 *= -1.0;
            }

            if particle.position.1 < -1.0 {
                particle.position.1 = -1.0;
                particle.velocity.1 *= -1.0;
            } else if particle.position.1 > 1.0 {
                particle.position.1 = 1.0;
                particle.velocity.1 *= -1.0;
            }
        }
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
        let gl: GL = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        Self::render_gl(self, gl);
    }
}

impl App {
    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .unwrap();
    }

    fn render_gl(&mut self, gl: GL) {
        let mut timestamp = 0.0;
        // create a simulation that can be passed to the render function
        let mut simulation = Simulation::new((900, 600));
        // get the position list from the simulation
        let positions: Vec<f32> = simulation
            .particles
            .iter()
            .flat_map(|p| vec![p.position.0, p.position.1])
            .collect();

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

        *cb.borrow_mut() = Some(Closure::wrap(Box::new({
            let cb = cb.clone();
            move || {
                // This should repeat every frame
                timestamp += 20.0;
                gl.uniform1f(time.as_ref(), timestamp as f32);

                // Update the simulation
                simulation.update(0.1);

                // Get the new positions
                let positions: Vec<f32> = simulation
                    .particles
                    .iter()
                    .flat_map(|p| vec![p.position.0, p.position.1])
                    .collect();

                // Update the vertex buffer
                let verts = js_sys::Float32Array::from(positions.as_slice());
                gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
                gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

                // Draw the new frame
                gl.draw_arrays(GL::POINTS, 0, 6);
                App::request_animation_frame(cb.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut()>));

        App::request_animation_frame(cb.borrow().as_ref().unwrap());
        
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());  
    yew::Renderer::<App>::new().render();
}