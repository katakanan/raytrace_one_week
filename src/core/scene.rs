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
    pub fn new(
        nw: u32,
        nh: u32,
        lookfrom: Point3<f64>,
        lookat: Point3<f64>,
        vup: Vector3<f64>,
        fov: f64,
        aspect: f64,
    ) -> Scene {
        let theta = fov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = lookfrom;

        let w = (lookfrom - lookat).normalize();
        let utmp = vup.cross(&w);
        let u = Vector3::new(utmp.x, utmp.y, utmp.z).normalize();
        let vtmp = w.cross(&u);
        let v = Vector3::new(vtmp.x, vtmp.y, vtmp.z).normalize();

        let lower_left_corner = origin - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;

        let screen = Screen {
            w: nw,
            h: nh,
            lower_left_corner,
            horizontal,
            vertical,
        };

        let camera = Camera {
            pos: Point3::origin(),
        };

        // let floor = Sphere {
        //     center: Point3::new(0.0, -100.5, -1.0),
        //     radius: 100.0,
        //     mat: Arc::new(Lambertian {
        //         albedo: Vector3::new(0.8, 0.8, 0.0),
        //     }),
        // };

        let R = (std::f64::consts::PI / 4.0).cos();

        let sphere1 = Sphere {
            center: Point3::new(-R, 0.0, -1.0),
            radius: R,
            mat: Arc::new(Lambertian {
                albedo: Vector3::new(0.0, 0.0, 1.0),
            }),
        };

        let sphere2 = Sphere {
            center: Point3::new(R, 0.0, -1.0),
            radius: R,
            mat: Arc::new(Lambertian {
                albedo: Vector3::new(1.0, 0.0, 0.0),
            }),
        };

        let mut shapes = ShapeList { v: vec![] };
        // shapes.v.push(floor);
        shapes.v.push(sphere1);
        shapes.v.push(sphere2);

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
