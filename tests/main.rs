use raygame::camera::Camera;
use raygame::color::Color;
use raygame::light::Light;
use raygame::ray::Ray;
use raygame::vector::Vector;

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

#[test]
fn ray_get_point() {
    let pos = Vector {
        x: 0.0,
        y: 0.707106781,
        z: 3.0,
    };
    let dir = Vector {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let ray = Ray { position: pos, direction: dir, t: 2.292893219, t_min: 0.0, t_max: 10000.0 };
    let point = Vector { x: 0.0, y: 0.707106781, z: 0.707106781 };
    assert!(ray.get_point()==point);
}

#[test]
fn camera_get_alpha() {
    let look_from = Vector { x: -1.0, y: 0.0, z: 0.0 };
    let look_at = Vector { x: 1.0, y: 0.0, z: 0.0 };
    let up = Vector { x: 0.0, y: 1.0, z: 0.0 };
    let camera = Camera {
        look_from: look_from,
        look_at: look_at,
        up: up,
        fovy: 30.0,
        fovx: 30.0,
        width: 100.0,
        height: 100.0,
    };
    assert!(camera.get_alpha(100.0) == 0.267949192);
}

#[test]
fn camera_get_beta() {
    let look_from = Vector { x: -1.0, y: 0.0, z: 0.0 };
    let look_at = Vector { x: 1.0, y: 0.0, z: 0.0 };
    let up = Vector { x: 0.0, y: 1.0, z: 0.0 };
    let camera = Camera {
        look_from: look_from,
        look_at: look_at,
        up: up,
        fovy: 30.0,
        fovx: 30.0,
        width: 100.0,
        height: 100.0,
    };
    assert!(camera.get_beta(0.0) == 0.267949192);
}

#[test]
fn color_new() {
    let color = Color::new(0.5, 0.4, 0.3);
    assert!(color.r == 127);
    assert!(color.g == 102);
    assert!(color.b == 76);
}

#[test]
fn light_generate_shadow_ray() {
    let local_point = Vector::new(0.0, 0.0, 0.0);
    let light = Light { position: Vector::new(0.0, 0.0, 5.0), color: Color::new(0.5, 0.5, 0.5) };
    let shadow_ray = light.generate_shadow_ray(local_point);
    // println!("shadow_ray.position.z {}", shadow_ray.position.z);
    assert!(shadow_ray.position == Vector::new(0.0, 0.0, 0.00049999997));
    assert!(shadow_ray.direction == Vector::new(0.0, 0.0, 5.0));
}
