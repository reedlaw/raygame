use crate::vector::Vector;

const T_MIN: f32 = 0.01;
const T_MAX: f32 = 1000000000.0;

#[derive(Debug)]
pub struct Ray {
    pub position: Vector,
    pub direction: Vector,
    pub t: f32,
    pub t_max: f32,
    pub t_min: f32,
}

impl Ray {
    pub fn new(position: Vector, direction: Vector) -> Ray {
        return Ray {
            position: position,
            direction: direction,
            t: 1.0,
            t_min: T_MIN,
            t_max: T_MAX,
        }
    }

    pub fn get_point(self) -> Vector {
        return self.position + (self.direction*self.t);
    }
}
