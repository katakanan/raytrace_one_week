extern crate image;
extern crate nalgebra as na;

use image::RgbaImage;
use na::{Point3, Vector3};
use std::sync::Arc;
use std::thread;

mod core;
use self::core::basic::Childarea;
use self::core::color::Color;
use self::core::scene::Scene;

pub fn render_child(child: &Childarea, scene: &Scene, sample: u32) -> (u32, RgbaImage) {
    let (w, h) = (child.w, child.h);

    let mut img = RgbaImage::new(w, h);
    for y in child.start..h + child.start {
        for x in 0..w {
            let mut c = Color::new(0.0, 0.0, 0.0);
            for _ in 0..sample {
                let u = (x as f64 + rand::random::<f64>()) / (w as f64);
                let v = (y as f64 + rand::random::<f64>()) / (child.par_h as f64);

                let ray = scene.get_ray(u, v);

                c = c + scene.color(&ray, 0);
            }
            c = c / (sample as f64);
            img.put_pixel(x, y - child.start, c.to_rgba(0.7))
        }
    }
    (child.start, img)
}

pub fn render(w: u32, h: u32, thread: u32, sample: u32) -> RgbaImage {
    let ch = ((h as f32) / (thread as f32)) as u32;

    let mut areas = vec![];

    for t in 0..thread {
        areas.push(Childarea {
            par_w: w,
            par_h: h,
            w: w,
            h: if t == (thread - 1) {
                h - ch * (thread - 1)
            } else {
                ch
            },
            start: t * ch,
        });
    }
    let lookfrom = Point3::new(3.0, 3.0, 2.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).norm();
    let aperture = 2.0;

    let scene = Arc::new(Scene::new(
        w,
        h,
        lookfrom,
        lookat,
        vup,
        20.0,
        (w as f64) / (h as f64),
        aperture,
        dist_to_focus,
    ));
    let mut children = vec![];

    for t in 0..thread {
        let s = scene.clone();
        let area = areas[t as usize];
        children.push(thread::spawn(move || render_child(&area, &s, sample)));
    }

    let mut imgs: Vec<(u32, RgbaImage)> = vec![];
    for child in children {
        imgs.push(child.join().unwrap());
    }
    let mut img = RgbaImage::new(w, h);
    for cimg in imgs {
        let (cw, ch) = (cimg.1).dimensions();

        let start = cimg.0;
        for y in 0..ch {
            for x in 0..cw {
                img.put_pixel(x, h - (y + start) - 1, *cimg.1.get_pixel(x, y));
            }
        }
    }

    img
}
