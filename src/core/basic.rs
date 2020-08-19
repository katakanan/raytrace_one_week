use na::{Point3, Vector3};
use std::sync::Arc;

use super::ray::Ray;
use super::shape_trait::Material;

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
    pub mat: Arc<dyn Material>,
}

#[derive(Debug, Clone, Copy)]
pub struct ScatterRecord {
    pub r: Ray,
    pub attenuation: Vector3<f64>,
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

pub fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(&n) * n
}
