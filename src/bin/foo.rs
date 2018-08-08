extern crate utils;

use utils::lyon;
use utils::image::*;
use lyon::path::builder::*;
use lyon::geom::math::*;
use lyon::svg::path_utils::PathSerializer;
use std::env;
use std::f32::consts::PI;

fn main() {

    let img_path = env::args().nth(1).expect("Need an input image");
    let img = utils::load_image(&img_path).expect("failed to load the image");

    let threshold: f32 = env::args().nth(2).unwrap_or("0.5".to_string()).parse().unwrap();

    let (w, h) = img.dimensions();

    let w = w as f32;
    let h = h as f32;
    let center = point(w * 0.5, h * 0.5);
    let mut builder = lyon::path::default::Path::builder();
    builder.move_to(center);
    const N: u32 = 1000000;
    let n = N as f32;
    let mut active = false;
    let mut d = 0;
    let wh = f32::max(w, h);
    for i in 1..N {
        let t = i as f32 / n;
        let v = vector(
            f32::cos(2000.0 * t) * (t + t*t) * wh * 0.6,
            f32::sin(2000.0 * t) * (t + t*t) * wh * 0.6,
        );
        let p = center + v;

        let intensity = {
            if p.x >= w || p.y >= h || p.x < 0.0 || p.y < 0.0 {
                0.0
            } else {
                1.0 - img.get_pixel(p.x as u32, p.y as u32).to_luma().data[0] as f32 / 255.0
            }
        };

        //intensity = (intensity - 0.5 ) * 2.0;
        //if intensity < 0.0 { intensity = 0.0 };
        //let p2 = p + v.normalize() * f32::sin((t * 50000.0) % (2.0 * PI)) * 20.0 * intensity;
        //if p2.x.is_nan() || p2.y.is_nan() {
        //    continue;
        //}
        //builder.line_to(p2);

        if intensity > threshold {
            if active {
                builder.line_to(p)
            } else {
                builder.move_to(p);
            }
            active = true;
            d = 0;
        } else {
            active = false;
            //if intensity > 0.2 {
            //    match d {
            //        5 => { builder.move_to(p); }
            //        6...10 => { builder.line_to(p); }
            //        11 => { d = 0; }
            //        _ => {}
            //    }
            //    d += 1;
            //} else {
            //    d = 0;
            //}
        }
    }

    let path = builder.build();

    let mut svg_out = PathSerializer::new();
    for event in path.iter() {
        svg_out.path_event(event);
    }

    println!("{}", svg_out.build());
}
