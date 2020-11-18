use crate::vector::Vector;

#[derive(Debug)]
pub struct Ray {
    pub position: Vector,
    pub direction: Vector,
    pub t: f32,
    pub t_max: f32,
    pub t_min: f32,
}

impl Ray {
    pub fn get_point(self) -> Vector {
        return self.position + (self.direction*self.t);
    }
}
