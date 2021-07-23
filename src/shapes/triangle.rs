use super::vector2::{Vector2, Vertex};

pub struct Triangle {
    pub vertex1: Vector2,
    pub vertex2: Vector2,
    pub vertex3: Vector2
}

impl Triangle {
    pub fn new(vertex1: Vec<i32>, vertex2: Vec<i32>, vertex3: Vec<i32>) -> Triangle {
        return Triangle { 
            vertex1: Vector2::new(vertex1[0] as f32 / 100f32, vertex1[1] as f32 / 100f32),
            vertex2: Vector2::new(vertex2[0] as f32 / 100f32, vertex2[1] as f32 / 100f32),
            vertex3: Vector2::new(vertex3[0] as f32 / 100f32, vertex3[1] as f32 / 100f32),
        }
    }
    pub fn to_vertices(self) -> Vec<Vertex> {
        return vec![
            self.vertex1.to_vertices(),
            self.vertex2.to_vertices(),
            self.vertex3.to_vertices()
        ]
    }
    pub fn add_to_constant(self) {

    }
    pub fn add_to_dynamic(self) {

    }
}