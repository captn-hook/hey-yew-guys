
#[allow(unused)]
use web_sys::console::info;
use yew::prelude::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, WebGl2RenderingContext as GLC};

//rc refcell for interior mutability
use std::cell::RefCell;
use std::rc::Rc;

// Wrap gl in Rc (Arc for multi-threaded) so it can be injected into the render-loop closure.
pub struct GlMgr {
    //node_ref is a reference to the canvas element
    context: GLC,
    cb: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
    refrence_time: Rc<RefCell<f64>>,
}

impl GlMgr {
    pub fn new(context: GLC, cb: Rc<RefCell<Option<Closure<dyn FnMut()>>>>, refrence_time: Rc<RefCell<f64>>,) -> Self {
        Self {
            context,
            cb,
            refrence_time,
        }
    }

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    fn render_gl(&mut self) {
        // This should log only once -- not once per frame
        
        let mut timestamp = 0.0;
       
        let vert_code = include_str!("./basic.vert");
        let frag_code = include_str!("./basic.frag");

        // This list of vertices will draw two triangles to cover the entire canvas, we will scale it and place it for each pixel
        let vertices: [f32; 12] = [
            -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        ];

        let count = 12;
        let size = 2;

        let vertex_buffer = self.context.create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(&vertices[..]);

        self.context.bind_buffer(GLC::ARRAY_BUFFER, Some(&vertex_buffer));
        self.context.buffer_data_with_array_buffer_view(GLC::ARRAY_BUFFER, &verts, GLC::STATIC_DRAW);

        let vert_shader = self.context.create_shader(GLC::VERTEX_SHADER).unwrap();
        self.context.shader_source(&vert_shader, vert_code);
        self.context.compile_shader(&vert_shader);

        let frag_shader = self.context.create_shader(GLC::FRAGMENT_SHADER).unwrap();
        self.context.shader_source(&frag_shader, frag_code);
        self.context.compile_shader(&frag_shader);

        let shader_program = self.context.create_program().unwrap();
        self.context.attach_shader(&shader_program, &vert_shader);
        self.context.attach_shader(&shader_program, &frag_shader);
        self.context.link_program(&shader_program);

        self.context.use_program(Some(&shader_program));

        // Attach the position vector as an attribute for the GLC context.
        let position = self.context.get_attrib_location(&shader_program, "a_position") as u32;
        self.context.vertex_attrib_pointer_with_i32(position, size, GLC::FLOAT, false, 0, 0);
        self.context.enable_vertex_attrib_array(position);

        // Attach the time as a uniform for the GLC context.
        let time = self.context.get_uniform_location(&shader_program, "u_time");
        self.context.uniform1f(time.as_ref(), timestamp as f32);

        self.context.draw_arrays(GLC::TRIANGLES, 0, count);

        // Gloo-render's request_animation_frame has this extra closure
        // wrapping logic running every frame, unnecessary cost.
        // Here constructing the wrapped closure just once.

        self.cb = Rc::new(RefCell::new(None));
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
                self.context.uniform1f(time.as_ref(), timestamp as f32);
                self.context.draw_arrays(GLC::TRIANGLES, 0, count);
                GlMgr::request_animation_frame(cb.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut()>));
        //click handler 
    
        GlMgr::request_animation_frame(self.cb.borrow().as_ref().unwrap());
    }
}