mod vector;

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

fn build_sample(i: f32, j: f32) -> Sample {
    Sample {
        x: j + 0.5,
        y: i + 0.5,
    }
}

fn main() {
    println!("Hello, world!");
}
