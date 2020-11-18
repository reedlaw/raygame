use crate::vector::Vector;
use crate::ray::Ray;
use crate::sample::Sample;

#[derive(Debug)]
pub struct Camera {
    pub look_from: Vector,
    pub look_at: Vector,
    pub up: Vector,
    pub fovx: f32,
    pub fovy: f32,
    pub width: f32,
    pub height: f32,
}

impl Camera {
    pub fn generate_ray(&self, sample: Sample) -> Ray {
        let a = self.look_from - self.look_at;
        let b = self.up;
        let w = a.normalize();
        let u = b.cross(w).normalize();
        let v = w.cross(u);
        let alpha = self.get_alpha(sample.x);
        let beta = self.get_beta(sample.y);
        let direction = ((u*alpha)+(v*beta)-w).normalize();
        Ray {
            position: self.look_from,
            direction: direction,
            t: 1.0,
            t_min: 0.01,
            t_max: 1000000000.0,
        }
    }

    pub fn get_alpha(&self, j: f32) -> f32 {
        let radians = self.fovx * 3.14159265 / 180.0;
        return (radians/2.0).tan() * ( (j-(self.width/2.0)) / (self.width/2.0) );
    }

    pub fn get_beta(&self, i: f32) -> f32 {
        let radians = self.fovx * 3.14159265 / 180.0;
        return (radians/2.0).tan() * ( ((self.width/2.0)-i) / (self.width/2.0) );
    }        
}
