use na::Vector3;

use super::basic::{random_in_unit_shpere, reflect, HitRecord, ScatterRecord};
use super::ray::Ray;
use super::shape_trait::Material;

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Vector3<f64>,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hr: &HitRecord) -> Option<ScatterRecord> {
        let target = hr.p + hr.n + random_in_unit_shpere();
        Some(ScatterRecord {
            r: Ray::new(hr.p, target - hr.p),
            attenuation: self.albedo,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    pub fuzz: f64,
    pub albedo: Vector3<f64>,
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<ScatterRecord> {
        let reflected =
            reflect(&(r.dir / r.dir.norm()), &(hr.n)) + self.fuzz * random_in_unit_shpere();

        if reflected.dot(&(hr.n)) > 0.0 {
            Some(ScatterRecord {
                r: Ray::new(hr.p, reflected),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
