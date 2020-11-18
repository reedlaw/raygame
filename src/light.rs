use crate::color::Color;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Light {
    pub position: Vector,
    pub color: Color,
}

impl Light {
    pub fn generate_shadow_ray(&self, local_point: Vector) -> Ray {
        let mut local_ray = Ray::new(local_point, self.position);
        local_ray.t = 0.0001;
        let offset_point = local_ray.get_point();
        return Ray::new(offset_point, self.position);
    }
}
