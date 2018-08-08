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

    let theta0: f32 = env::args().nth(1).unwrap_or("0.5".to_string()).parse().unwrap();
    let theta1: f32 = env::args().nth(2).unwrap_or("0.5".to_string()).parse().unwrap();
    let theta2: f32 = env::args().nth(3).unwrap_or("0.5".to_string()).parse().unwrap();
    let dtheta: f32 = env::args().nth(4).unwrap_or("0.5".to_string()).parse().unwrap();
    let n: u32 = env::args().nth(5).unwrap_or("100".to_string()).parse().unwrap();

    let (w, h) = (200.0, 200.0);

    let w = w as f32;
    let h = h as f32;
    let center = point(w * 0.5, h * 0.5);
    let mut builder = lyon::path::default::Path::builder();
    builder.move_to(center);
    const N: u32 = 1000000;

    let mut radius = 300.0;

    let mut t = 0.0;
    builder.move_to(
        point(
            center.x + radius * f32::cos(theta0),
            center.y + radius * f32::sin(theta0),
        )
    );
    for i in 0..n {
        builder.line_to(point(
            center.x + radius * f32::cos(theta0 + t),
            center.y + radius * f32::sin(theta0 + t),
        ));
        builder.line_to(point(
            center.x + radius * f32::cos(theta1 + t),
            center.y + radius * f32::sin(theta1 + t),
        ));
        builder.line_to(point(
            center.x + radius * f32::cos(theta2 + t),
            center.y + radius * f32::sin(theta2 + t),
        ));
        t += 2.0 * PI * dtheta / (n as f32);
    }
radius /= 1.91;

    builder.move_to(
        point(
            center.x + radius * f32::cos(theta0 + t),
            center.y + radius * f32::sin(theta0 + t),
        )
    );
        for i in 0..n {
        builder.line_to(point(
            center.x + radius * f32::cos(theta0 + t),
            center.y + radius * f32::sin(theta0 + t),
        ));
        builder.line_to(point(
            center.x + radius * f32::cos(theta1 + t),
            center.y + radius * f32::sin(theta1 + t),
        ));
        builder.line_to(point(
            center.x + radius * f32::cos(theta2 + t),
            center.y + radius * f32::sin(theta2 + t),
        ));
        radius /= 1.01;

        t += 1.0 * PI * dtheta / (n as f32);
    }

    let path = builder.build();

    let mut svg_out = PathSerializer::new();
    for event in path.iter() {
        svg_out.path_event(event);
    }

    println!("{}", svg_out.build());
}
