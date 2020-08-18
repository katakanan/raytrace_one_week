use na::{Point3, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Childarea {
    pub par_w: u32,
    pub par_h: u32,
    pub w: u32,
    pub h: u32,
    pub start: u32,
}

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point3<f64>,
    pub n: Vector3<f64>,
}
