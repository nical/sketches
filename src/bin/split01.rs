extern crate utils;

use utils::lyon;
use utils::lyon::algorithms::{splitter, advanced_path::*};
use utils::image::*;
use lyon::path::PathEvent;
use lyon::path::builder::*;
use lyon::path::default::*;
use lyon::path::iterator::Transformed;
use lyon::geom::math::*;
use lyon::geom::LineSegment;
use lyon::svg::path_utils::PathSerializer;
use std::env;
use std::f32::consts::PI;

fn circle(center: Point, r: f32, path: &mut Builder) {
    path.move_to(center + vector(r, 0.0));
    for a in 1..16u32 {
        let angle = a as f32 / 16.0 * 2.0 * PI;
        path.line_to(center + vector(
            f32::cos(angle) * r,
            f32::sin(angle) * r,
        ));
    }
    path.close();
}

fn main() {
    let seed: usize = env::args().nth(1).unwrap_or("42".to_string()).parse().unwrap();
    let n_splits: u32 = env::args().nth(2).unwrap_or("5".to_string()).parse().unwrap();
    let spread: f32 = env::args().nth(3).unwrap_or("1.0".to_string()).parse().unwrap();

    let w: f32 = 29.7;
    let h: f32 = 21.0;

    let center: Point = point(w/2.0, h/2.0);

    let mut splitter = splitter::Splitter::new();
    let mut random = utils::Random::new(seed);
    let mut path = AdvancedPath::new();
    let mut debug_path = lyon::path::default::Path::builder();


    let mut initial_poly = Vec::new();
    let mut angle = 0.0;
    let r = h / 2.2;
    while angle <= 2.0 * PI {
        let mut da = random.get_f32(0.05..0.2);

        initial_poly.push(center + vector(
            r * f32::cos(angle),
            r * f32::sin(angle),
        ));

        angle += da;
    }

    let sp = path.add_polyline(
        &initial_poly[..],
        true,
    );

    for i in 0..n_splits {
        let angle = random.get_f32(0.1..2.0*PI);
        let c = center + vector(
            random.get_f32(-r*0.8..r*0.8),
            random.get_f32(-r*0.8..r*0.8),
        );
        let a = vector(f32::cos(angle), f32::sin(angle));
        let segment = LineSegment {
            from: c + a * 50.0,
            to: c - a * 50.0,
        };

        debug_path.move_to(segment.from);
        debug_path.line_to(segment.to);
        //eprintln!("split line: {:?}", line);
        //circle(line.point, 0.5, &mut debug_path);
        let nth = random.get_u32(0..(path.sub_path_ids().len() as u32)) as u16;
        let sp = path.sub_path_ids().nth(nth);
        let ids = splitter.split_with_segment(&mut path, &sp, &segment);
    }

    let mut svg_out = PathSerializer::new();

    for sp in path.sub_path_ids() {
        let mut c: Point = point(0.0, 0.0);
        let mut n = 0.0;
        for evt in path.sub_path_edges(sp).path_iter() {
            match evt {
                PathEvent::MoveTo(p) => { c += p.to_vector(); n += 1.0; }
                PathEvent::LineTo(p) => { c += p.to_vector(); n += 1.0; }
                _ => {}
            }
        }
        let d = (c - center) / n;
        let l = d.length();
        let t = d.normalize() * l * l * 0.01 * spread;
        let mut transform = Transform2D::identity();
        transform = transform.post_translate(-d);
        transform = transform.post_rotate(Angle::radians(random.get_f32(-0.1..0.1)));
        transform = transform.post_translate(d);
        transform = transform.post_translate(t);
        for event in Transformed::new(&transform, path.sub_path_edges(sp).path_iter()) {
            svg_out.path_event(event);
        }
    }

    let debug_path = debug_path.build();
    //for event in debug_path.iter() {
    //    svg_out.path_event(event);
    //}

    println!("{}", svg_out.build());
}