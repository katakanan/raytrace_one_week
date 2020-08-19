use std::sync::Arc;

use na::{Point3, Vector3};

// use super::basic::random_in_unit_shpere;
use super::camera::Camera;
use super::color::Color;
use super::material::{Dielectric, Lambertian, Metal};
use super::ray::Ray;
use super::screen::Screen;
use super::shape_trait::HIT;
use super::shapelist::ShapeList;
use super::sphere::Sphere;

#[derive(Debug, Clone)]
pub struct Scene {
    pub screen: Screen,
    pub camera: Camera,
    pub shapes: ShapeList,
}

impl Scene {
    pub fn new(w: u32, h: u32) -> Scene {
        let lower_left_corner = Point3::new(-2.0, -1.0, -1.0);
        let horizontal = Vector3::new(4.0, 0.0, 0.0);
        let vertical = Vector3::new(0.0, 2.0, 0.0);

        let screen = Screen {
            w,
            h,
            lower_left_corner,
            horizontal,
            vertical,
        };

        let camera = Camera {
            pos: Point3::origin(),
        };

        let floor = Sphere {
            center: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            mat: Arc::new(Lambertian {
                albedo: Vector3::new(0.8, 0.8, 0.0),
            }),
        };

        let sphere1 = Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            mat: Arc::new(Lambertian {
                albedo: Vector3::new(0.1, 0.2, 0.5),
            }),
        };

        let sphere2 = Sphere {
            center: Point3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            mat: Arc::new(Metal {
                fuzz: 0.2,
                albedo: Vector3::new(0.8, 0.6, 0.2),
            }),
        };

        let sphere3 = Sphere {
            center: Point3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            mat: Arc::new(Dielectric { ref_idx: 1.5 }),
        };

        let mut shapes = ShapeList { v: vec![] };
        shapes.v.push(floor);
        shapes.v.push(sphere1);
        shapes.v.push(sphere2);
        shapes.v.push(sphere3);

        Scene {
            screen,
            camera,
            shapes,
        }
    }

    pub fn color(&self, r: &Ray, depth: u32) -> Color {
        match self.shapes.hit(r, 0.001, std::f64::MAX) {
            Some(hr) => match (hr.mat.scatter(&r, &hr), depth < 50) {
                (Some(sr), true) => {
                    let ctmp = self.color(&(sr.r), depth + 1);
                    let r = ctmp.r * sr.attenuation.x;
                    let g = ctmp.g * sr.attenuation.y;
                    let b = ctmp.b * sr.attenuation.z;

                    Color::new(r, g, b)
                }
                (_, _) => Color::new(0.0, 0.0, 0.0),
            },
            None => {
                let unit_direction = r.dir / r.dir.norm();
                let t = 0.5 * (unit_direction.y + 1.0);
                Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
            }
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rdir = self.screen.lower_left_corner - self.camera.pos
            + (self.screen.horizontal * u + self.screen.vertical * v);

        Ray::new(self.camera.pos, rdir)
    }
}
