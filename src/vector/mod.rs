use std::ops;

#[derive(Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;
    
    fn add(self, _rhs: Vector) -> Vector {
        Vector {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, scalar: f32) -> Vector {
        if scalar == 0.0 {
            panic!("Cannot divide by zero!");
        }

        Vector {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl ops::Mul<Vector> for Vector {
    type Output = f32;

    fn mul(self, _rhs: Vector) -> f32 {
        return self.x * _rhs.x + self.y * _rhs.y + self.z * _rhs.z;
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, _rhs: Vector) -> Vector {
        Vector {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        let factor = 10.0f32.powi(8);
        (self.x * factor).trunc() == (other.x * factor).trunc() &&
            (self.y * factor).trunc() == (other.y * factor).trunc() &&
            (self.z * factor).trunc() == (other.z * factor).trunc()
    }
}

impl Eq for Vector {}

impl Vector {
    pub fn normalize(self) -> Vector {
        if self.x==0.0 && self.y==0.0 && self.z==0.0 {
            panic!("Cannot divide by zero!");
        }
        let length = ((self.x*self.x) + (self.y*self.y) + (self.z*self.z)).sqrt();
        return self/length;
    }

    pub fn cross(self, other: Vector) -> Vector {
        Vector {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x,
        }
    }
}
