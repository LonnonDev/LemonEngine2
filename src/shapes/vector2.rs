use std::ops::{Add, AddAssign, Sub, Mul, Div};

use glium::implement_vertex;

// A Vertex struct which is just [f32, f32]
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

// A vector struct that can be converted from Vector2 to Vertex
#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    // Creates a new Vectro2 from 2 f32's
    pub fn new(x: f32, y: f32) -> Vector2 {
        return Vector2 {
            x,
            y
        }
    }
    // Converts the Vector2 to a Vertex
    pub fn to_vertices(&self) -> Vertex {
        return Vertex { 
            position: [self.x, self.y]
        }
    }
}

impl Vertex {
    pub fn implement() {
        implement_vertex!(Vertex, position);
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        return Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Div for Vector2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        return Self {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }
}

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        return Self {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}
