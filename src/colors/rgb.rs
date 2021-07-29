use crate::colors::rgba::RGBA;

// A RGB color struct
pub struct RGB {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl RGB {
    /// Creates the Struct, from 3 f32's
    pub fn new(red: f32, green: f32, blue: f32) -> RGB {
        return RGB {
            red,
            green,
            blue,
        }
    }
    /// Normalizes the Value from 0-255 to 0-1
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
    /// Unnormalizes the values from 0-1 to 0-255
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
    /// Converts the struct from RGB to RGBA and takes alpha (f32) as a param
    pub fn to_rgba(&self, alpha: f32) -> RGBA {
        return RGBA {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha,
        }
    }
}
