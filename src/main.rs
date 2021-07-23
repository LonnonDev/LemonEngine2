pub mod colors;
pub mod shapes;
mod workings;
mod renderer;

use glium::{Display, glutin::{self, event_loop::EventLoop}, implement_vertex};
use shapes::triangle::Triangle;

use crate::{renderer::Renderer, shapes::vector2::Vertex, workings::on_frame::on_frame};

fn main() {
    implement_vertex!(Vertex, position);
    let mut renderer = Renderer::new();

    let triangle = Triangle::new(
        vec![-10,-10], 
        vec![0,10], 
        vec![10,-10]
    ).to_vertices();
    renderer.add_to_constant(triangle);
    let shape = renderer.combine();

    let (event_loop, display) = on_load();


    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    on_frame(event_loop, display, vertex_shader_src, fragment_shader_src, shape);
}

fn on_load() -> (EventLoop<()>, Display) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    return (event_loop, display)
}

