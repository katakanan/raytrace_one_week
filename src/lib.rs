extern crate image;
extern crate nalgebra as na;

use image::{Rgba, RgbaImage};
use std::sync::Arc;
use std::thread;

mod core;
use self::core::basic::Childarea;
use self::core::scene::Scene;

pub fn render_child(child: &Childarea, scene: &Scene) -> (u32, RgbaImage) {
    let (w, h) = (child.w, child.h);

    println!("{}", child.start);
    let mut img = RgbaImage::new(w, h);
    for y in child.start..h + child.start {
        for x in 0..w {
            let u = (x as f64) / (w as f64);
            let v = ((child.par_h - y - 1) as f64) / (child.par_h as f64);

            let ray = scene.get_ray(u, v);
            let col = scene.color(&ray);

            img.put_pixel(x, y - child.start, col.to_rgba(1.0))
        }
    }
    (child.start, img)
}

pub fn render(w: u32, h: u32, thread: u32) -> RgbaImage {
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

    let scene = Arc::new(Scene::new(w, h));
    let mut children = vec![];

    for t in 0..thread {
        let s = scene.clone();
        let area = areas[t as usize];
        children.push(thread::spawn(move || render_child(&area, &s)));
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
