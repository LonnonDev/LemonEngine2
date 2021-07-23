use crate::shapes::vector2::Vertex;

#[derive(Clone)]
pub struct Renderer {
    pub constant: Vec<Vertex>,
    pub dynamic: Vec<Vertex>
}

#[allow(dead_code)]
impl Renderer {
    pub fn new() -> Renderer {
        return Renderer {
            constant: vec![],
            dynamic: vec![]
        }
    }
    pub fn add_to_constant(&mut self, constant: Vec<Vertex>) {
        self.constant.extend(constant);
    }
    pub fn add_to_dynamic(&mut self, dynamic: Vec<Vertex>) {
        self.dynamic.extend(dynamic);
    }
    pub fn clear_constant(mut self) {
        self.constant = vec![];
    }
    pub fn clear_dynamic(mut self) {
        self.dynamic = vec![];
    }
    pub fn clear_both(mut self) {
        self.constant = vec![];
        self.dynamic = vec![];
    }
    pub fn combine(self) -> Vec<Vertex> {
        return [self.dynamic, self.constant].concat()
    }
}