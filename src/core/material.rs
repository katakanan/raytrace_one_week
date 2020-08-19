use na::Vector3;

use super::basic::{random_in_unit_shpere, reflect, refract, HitRecord, ScatterRecord};
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

#[derive(Debug, Clone)]
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Vector3::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt) = if ray.dir.dot(&hit.n) > 0.0 {
            (-hit.n, self.ref_idx)
        } else {
            (hit.n, 1.0 / self.ref_idx)
        };
        if let Some(refracted) = refract(&ray.dir, &outward_normal, ni_over_nt) {
            let scattered = Ray::new(hit.p, refracted);

            return Some(ScatterRecord {
                r: scattered,
                attenuation: attenuation,
            });
        }
        let reflected = reflect(&ray.dir, &hit.n);
        let scattered = Ray::new(hit.p, reflected);
        return Some(ScatterRecord {
            r: scattered,
            attenuation: attenuation,
        });
    }
}
