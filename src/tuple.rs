pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Tuple {
    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z, w: 0.0}
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z, w: 1.0}
    }
}