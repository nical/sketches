extern crate utils;

use utils::lyon;
//use utils::image::*;
use lyon::path::builder::*;
use lyon::geom::math::*;
use lyon::svg::path_utils::PathSerializer;
use std::env;
use std::ops::Range;
use std::f32::consts::PI;
use lyon::path::default::Builder;

fn step(x: f32, range: Range<f32>) -> f32 {
    if x < range.start {
        return 0.0;
    }
    if x > range.end {
        return 1.0;
    }

    (x - range.start) / (range.end - range.start)
}

fn shape(center: &Point, radius: f32, sides: f32, rot: f32, builder: &mut Builder) {
    let frac = sides % 1.0;
    let x = 1.0 / sides;
    let f = frac * x;
    let mut a = 0.0;
    builder.move_to(*center + vector(
        radius * f32::cos(a * 2.0 * PI + rot),
        radius * f32::sin(a * 2.0 * PI + rot),
    ));
    a += if f != 0.0 { f } else { x };
    let n = sides.ceil() as u32;
    for i in 1..n {
        builder.line_to(*center + vector(
            radius * f32::cos(a * 2.0 * PI + rot),
            radius * f32::sin(a * 2.0 * PI + rot),
        ));
        a += x;
    }

    builder.close();
}

fn main() {

//    let img_path = env::args().nth(1).expect("Need an input image");
//    let img = utils::load_image(&img_path).expect("failed to load the image");

    let grid_step: f32 = env::args().nth(1).unwrap_or("5".to_string()).parse().unwrap();
    let radius: f32 = env::args().nth(2).unwrap_or("10".to_string()).parse().unwrap();
    let margin: f32 = env::args().nth(3).unwrap_or("10".to_string()).parse().unwrap();

    let (w, h) = (297.0, 210.0);
    let center = point(w, h) * 0.5;
    let mut builder = lyon::path::default::Path::builder();

    let mut y = margin;
    while y < h - margin {
        let mut x = margin;
        while x < w - margin {
            let p = point(x, y);
            let dist = (p - center).length();
            let swirl = f32::sin((p - center).angle_from_x_axis().radians + dist * 0.05);
            //println!("dist: {:?}", dist);
            //let sides = 4.0 + step((p - center).length() / 10.0, 5.0..10.0) * 3.0;
            let sides = 4.0 + step(dist, 85.0..150.0) * 15.0;
            //let rotation = x / w * PI;
            let rotation = step(dist, 0.0..120.0) * PI;
            //let rotation = swirl * PI * 0.25;
            //let radius = radius + step(dist, 100.0..140.0) * radius;
            let radius = radius * 0.2 + swirl.abs() * radius * 0.8;
            shape(&p, radius, sides, rotation, &mut builder);

            x += grid_step
        }
        y += grid_step
    }

    let path = builder.build();

    let mut svg_out = PathSerializer::new();
    for event in path.iter() {
        svg_out.path_event(event);
    }

    println!("{}", svg_out.build());
}
