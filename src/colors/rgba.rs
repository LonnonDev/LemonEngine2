use glium::{Frame, Surface};

use crate::colors::rgb::RGB;

pub struct RGBA {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl RGBA {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> RGBA {
        return RGBA {
            red,
            green,
            blue,
            alpha
        }
    }
    pub fn normalize(&mut self) -> RGBA {
        let red = self.red/255f32;
        let blue = self.blue/255f32;
        let green = self.green/255f32;
        let alpha = self.alpha/255f32;
        return RGBA {
            red,
            green,
            blue,
            alpha
        }
    }
    pub fn unnormalize(&mut self) -> RGBA {
        let red = self.red*255f32;
        let blue = self.blue*255f32;
        let green = self.green*255f32;
        let alpha = self.alpha*255f32;
        return RGBA {
            red,
            green,
            blue,
            alpha
        }
    }
    pub fn to_rgb(&self) -> RGB {
        return RGB {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }
    pub fn set_bg_color(&self, mut target: Frame) -> Frame {
        target.clear_color(self.red, self.green, self.blue, self.alpha);
        return target
    }
}
