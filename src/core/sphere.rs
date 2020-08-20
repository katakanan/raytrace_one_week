use na::Point3;
use std::sync::Arc;

use super::basic::HitRecord;
use super::ray::Ray;
use super::shape_trait::{Material, HIT};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}

impl HIT for Sphere {
    fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.dir.dot(&r.dir);
        let b = r.dir.dot(&oc);
        let c = oc.dot(&oc) - self.radius.powi(2);
        let d = b * b - a * c;

        if d > 0.0 {
            let sqrt_discriminant = d.sqrt();
            let tmp = (-b - sqrt_discriminant) / a;
            if t0 < tmp && tmp < t1 {
                let p = r.point_at_paramete(tmp);
                let n = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t: tmp,
                    p: p,
                    n: n,
                    mat: self.mat.clone(),
                });
            }
            let tmp = (-b + sqrt_discriminant) / a;
            if t0 < tmp && tmp < t1 {
                let p = r.point_at_paramete(tmp);
                let n = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t: tmp,
                    p: p,
                    n: n,
                    mat: self.mat.clone(),
                });
            }
        }
        None
    }
}
