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

pub fn random_in_unit_shpere() -> Vector3<f64> {
    let mut p;
    loop {
        p =
            2.0 * Vector3::new(
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
            ) - Vector3::new(1.0, 1.0, 1.0);
        if p.norm_squared() < 1.0 {
            break;
        }
    }
    p
}
