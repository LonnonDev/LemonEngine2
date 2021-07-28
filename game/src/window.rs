use glium::Display;

use crate::workings::renderer::Renderer;

// A Window struct to create the window
pub struct Window {
    pub display: Display, 
    pub vertex_shader_src: String,
    pub fragment_shader_src: String,
    pub renderer: Renderer
}


impl Window {
    // Creates a Window
    pub fn new(
        display: Display, 
        vertex_shader_src: String,
        fragment_shader_src: String,
    ) -> Window {
        return Window {
            display,
            vertex_shader_src,
            fragment_shader_src,
            renderer: Renderer::new(),
        }
    }
}