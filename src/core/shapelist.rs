use std::fmt;

use super::basic::HitRecord;
use super::ray::Ray;
use super::shape_trait::HIT;
use super::sphere::Sphere;

#[derive(Clone)]
pub struct ShapeList {
    // pub v: Vec<Arc<HIT>>,
    pub v: Vec<Sphere>,
}

impl HIT for ShapeList {
    fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
        let mut closet = t1;
        let mut ret: Option<HitRecord> = None;

        for p in &(self.v) {
            ret = match p.hit(&r, t0, closet) {
                None => ret,
                Some(hr) => {
                    closet = hr.t;
                    Some(hr)
                }
            };
        }
        ret
    }
}

impl fmt::Debug for ShapeList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ShapeList{{ shapelist }}")
    }
}
