use glium::{Display, Surface, glutin::{self, event_loop::EventLoop}};

use crate::{colors::rgba::RGBA, shapes::vector2::Vertex};

pub fn on_frame(
    event_loop: EventLoop<()>, 
    display: Display, 
    vertex_shader_src: &str,
    fragment_shader_src: &str,
    renderer: Vec<Vertex>,
) {
    let program = glium::Program::from_source(
        &display, 
        vertex_shader_src, 
        fragment_shader_src, 
        None)
    .unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    event_loop.run(move |event, _, control_flow| {
        let vertex_buffer = glium::VertexBuffer::new(&display, &renderer).unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
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

        let mut target = display.draw();

        target = RGBA::new(241.0, 156.0, 187.0, 255.0).normalize().set_bg_color(target);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,&Default::default()).unwrap();
        target.finish().unwrap();
    });
}