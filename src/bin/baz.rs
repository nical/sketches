extern crate utils;

use utils::lyon;
use utils::image::*;
use lyon::path::builder::*;
use lyon::geom::math::*;
use lyon::svg::path_utils::PathSerializer;
use std::env;
use std::f32::consts::PI;

fn main() {

//    let img_path = env::args().nth(1).expect("Need an input image");
//    let img = utils::load_image(&img_path).expect("failed to load the image");

    let n: u32 = env::args().nth(1).unwrap_or("100".to_string()).parse().unwrap();

    let (w, h) = (200.0, 200.0);

    let w = w as f32;
    let h = h as f32;
    let center = point(w * 0.5, h * 0.5);
    let mut builder = lyon::path::default::Path::builder();
    builder.move_to(center);
    const N: u32 = 1000000;

    let mut radius = 20.0;
    let mut p = point(0.0, 0.0);

    let mut theta = 1.0;
    let mut t = 0.0;

    builder.move_to(p);
    for i in 0..n {
        let v = vector(
            radius * f32::cos(theta),
            radius * f32::sin(theta),
        );
        p += v;
        if (p - center).length() > 200.0 {
            theta += 1.4;
            let v = vector(
                radius * f32::cos(theta),
                radius * f32::sin(theta),
            );
            p += v;
        }
        builder.line_to(p);
        theta += f32::sin(t * t * t) * 0.2;
        t += 1.0;
    }

    let path = builder.build();

    let mut svg_out = PathSerializer::new();
    for event in path.iter() {
        svg_out.path_event(event);
    }

    println!("{}", svg_out.build());
}
