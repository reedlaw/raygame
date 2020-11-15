use std::ops;

#[derive(Debug)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
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
    fn normalize(self) -> Vector {
        if self.x==0.0 && self.y==0.0 && self.z==0.0 {
            panic!("Cannot divide by zero!");
        }
        let length = ((self.x*self.x) + (self.y*self.y) + (self.z*self.z)).sqrt();
        return self/length;
    }

    fn cross(self, other: Vector) -> Vector {
        Vector {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x,
        }
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

    #[test]
    fn vector_subtract() {
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
            x: 0.0,
            y: 0.4,
            z: 0.5,
        };
        assert!(v1-v2==sum);
    }

    #[test]
    fn vector_divide() {
        let vector = Vector {
            x: 0.1,
            y: 0.2,
            z: 0.5,
        };
        let scalar = 0.1;
        let result = Vector {
            x: 1.0,
            y: 2.0,
            z: 5.0,
        };
        assert!(vector/scalar==result);
    }

    #[test]
    fn vector_dot_product() {
        let v1 = Vector {
            x: -4.0,
            y: -4.0,
            z: 4.0,
        };
        let v2 = Vector {
            x: -4.0,
            y: -4.0,
            z: 4.0,
        };
        let result = 48.0;
        assert!(v1*v2==result);
    }

    #[test]
    fn vector_multiply_by_scalar() {
        let vector = Vector {
            x: 0.1,
            y: 0.2,
            z: 0.5,
        };
        let scalar = 0.1;
        let result = Vector {
            x: 0.01,
            y: 0.02,
            z: 0.05,
        };
        assert!(vector*scalar==result);
    }

    #[test]
    fn vector_cross() {
        let v1 = Vector {
            x: 0.5,
            y: 0.0,
            z: 0.5,
        };
        let v2 = Vector {
            x: 0.0,
            y: 0.5,
            z: 0.0,
        };
        let result = Vector {
            x: -0.25,
            y: 0.0,
            z: 0.25,
        };
        assert!(v1.cross(v2)==result);
    }

    #[test]
    fn vector_normalize() {
        let vector = Vector {
            x: 3.0,
            y: 1.0,
            z: 2.0,
        };
        let result = Vector {
            x: 0.80178368091583252,
            y: 0.26726123690605164,
            z: 0.53452247381210327,
        };
        assert!(vector.normalize()==result);
    }
}
