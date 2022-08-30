use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, WebGlRenderingContext as GL, WebGlRenderingContext};
use yew::{html, Component, Context, Html, NodeRef};

// Wrap gl in Rc (Arc for multi-threaded) so it can be injected into the render-loop closure.
pub struct App {
    node_ref: NodeRef,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas ref={self.node_ref.clone()} />
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
        // Once rendered, store references for the canvas and GL context. These can be used for
        // resizing the rendering area when the window or canvas element are resized, as well as
        // for making GL calls.
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
        let gl: GL = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        Self::render_gl(gl);
    }
}

impl App {
    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    fn render_gl(gl: WebGlRenderingContext) {
        // This should log only once -- not once per frame

        let mut timestamp = 0.0;

        let vert_code = include_str!("./basic.vert");
        let frag_code = include_str!("./basic.frag");

        // This list of vertices will draw two triangles to cover the entire canvas.
        let vertices: Vec<f32> = vec![
            -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        ];
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

        gl.draw_arrays(GL::TRIANGLES, 0, 6);

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
                gl.draw_arrays(GL::TRIANGLES, 0, 6);
                App::request_animation_frame(cb.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut()>));

        App::request_animation_frame(cb.borrow().as_ref().unwrap());
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
