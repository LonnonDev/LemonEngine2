pub mod colors;
pub mod shapes;
mod workings;
mod window;

use std::{fs, time};

use colors::rgba::RGBA;
use glium::{Surface, glutin::{self, event_loop::EventLoop}, implement_vertex};

use crate::{shapes::{triangle::Triangle, vector2::Vertex}, window::Window};

#[allow(dead_code)]
impl Window {
    fn on_load(self) {
        
    }
    fn on_frame(mut self, event_loop: EventLoop<()>) {
        let program = glium::Program::from_source(
            &self.display, 
            self.vertex_shader_src.as_str(), 
            self.fragment_shader_src.as_str(), 
            None)
        .unwrap();
        let bg_color = RGBA::new(241.0, 156.0, 187.0, 255.0).normalize();
    
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    
        

        event_loop.run( move |event, _, control_flow| {
            self.renderer.clear_dynamic();
            let triangle = Triangle::new(
                vec![-10,-10], 
                vec![0,10], 
                vec![10,-10]
            ).to_vertices();
            self.renderer.add_to_dynamic(triangle);
            let vertex_buffer = glium::VertexBuffer::new(&self.display, &self.renderer.combine()).unwrap();
    
            let next_frame_time = time::Instant::now() +
                time::Duration::from_nanos(16_666_667);
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    
            match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                    _ => return,
                },
                glutin::event::Event::NewEvents(cause) => match cause {
                    glutin::event::StartCause::ResumeTimeReached { .. } => (),
                    glutin::event::StartCause::Init => (),
                    _ => return,
                },
                _ => return,
            }
    
            let mut target = self.display.draw();
    
            target = bg_color.set_bg_color(target);
            target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,&Default::default()).unwrap();
            target.finish().unwrap();
        });
    }
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_shader_src = fs::read_to_string("shader.vert")
        .expect("Something went wrong reading the file");
    let fragment_shader_src = fs::read_to_string("shader.frag")
        .expect("Something went wrong reading the file");

    implement_vertex!(Vertex, position);

    let window = Window::new(
        display,
        vertex_shader_src,
        fragment_shader_src
    );
    window.on_frame(event_loop);
}