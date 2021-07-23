use crate::colors::rgba::RGBA;

pub struct RGB {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl RGB {
    pub fn new(red: f32, green: f32, blue: f32) -> RGB {
        return RGB {
            red,
            green,
            blue,
        }
    }
    pub fn normalize(&mut self) -> RGB {
        let red = self.red/255f32;
        let blue = self.blue/255f32;
        let green = self.green/255f32;
        return RGB {
            red,
            green,
            blue,
        }
    }
    pub fn unnormalize(&mut self) -> RGB {
        let red = self.red*255f32;
        let blue = self.blue*255f32;
        let green = self.green*255f32;
        return RGB {
            red,
            green,
            blue,
        }
    }
    pub fn to_rgba(&self, alpha: f32) -> RGBA {
        return RGBA {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha,
        }
    }
}
