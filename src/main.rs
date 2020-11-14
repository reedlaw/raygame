use std::ops;

struct Camera {
}

struct Point {
    x: f32,
    y: f32,
    z: f32,
}

struct Sample {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;
    
    fn add(self, _rhs: Vector) -> Vector {
        let vector = Vector {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        };
        return vector;
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

fn build_sample(i: f32, j: f32) -> Sample {
    Sample {
        x: j + 0.5,
        y: i + 0.5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn vector_eq() {
        let v1 = Vector {
            x: 0.15 + 0.15 + 0.15,
            y: 0.000000001,
            z: 0.500000002,
        };
        let v2 = Vector {
            x: 0.1 + 0.1 + 0.25,
            y: -0.000000001,
            z: 0.500000001,
        };
        assert!(v1==v2);
    }

    #[test]
    fn vector_add() {
        let v1 = Vector {
            x: 0.1,
            y: 0.2,
            z: 0.5,
        };
        let v2 = Vector {
            x: 0.1,
            y: -0.2,
            z: 0.0,
        };
        let sum = Vector {
            x: 0.2,
            y: 0.0,
            z: 0.5,
        };
        assert!(v1+v2==sum);
    }    
}

fn main() {
    println!("Hello, world!");
}
