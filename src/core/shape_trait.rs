use super::basic::HitRecord;
use super::ray::Ray;
use std::fmt;

pub trait HIT: fmt::Debug + Sync + Send {
    fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord>;
}
