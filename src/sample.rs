pub struct Sample {
    pub x: f32,
    pub y: f32,
}

impl Sample {
    pub fn new(i: f32, j: f32) -> Sample {
        Sample {
            x: i+0.5,
            y: j+0.5,
        }
    }
}
