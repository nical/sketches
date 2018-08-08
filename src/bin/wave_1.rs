extern crate utils;
use utils::*;

use lyon::path::builder::*;
use lyon::geom::math::*;
use lyon::geom::*;

fn main() {
    let mut params = Params::new();
    let seed: usize = params.get_usize(42);
    let mut rng = Random::new(seed);

    let margin = 10.0;
    let (w, h) = (297.0, 210.0);
    let mut builder = lyon::path::default::Path::builder();

    let a = point(0.05 * while, 0.6 * h);
    let b = point(0.90 * w, 0.2 * h);


    let mut sign = 1.0;
    let v = vector(1.0, 1.0);
    let adv = vector(1.0, -1.0).normalize();
    let mut p = a;

    let arc = Arc {
        center: p - v.normalize() * 5.0,
        radii: vector(5.0, 5.0),
        start_angle: Angle::zero(),
        sweep_angle: Angle::pi() * 2.0,
        x_rotation: Angle::zero(),
    };
    builder.move_to(arc.sample(0.0));
    for i in 0..100 {
        builder.line_to(arc.sample(i as f32 / 100.0));
    }

    builder.move_to(p);

    loop {
        let (dir, y_max) = if sign >= 0.0 {
            (v, h - margin)
        } else {
            (-v, margin)
        };

        let main_line = LineSegment {
            from: a,
            to: b,
        };

        let border = Line { point: point(0.0, y_max), vector: vector(1.0, 0.0) };
        let l = Line { point: p, vector: dir };
        let border_intersection = l.intersection(&border).unwrap();

        let pl = l.intersection(&main_line.to_line()).unwrap();
        let remaining = (pl - b).length();

        if remaining < 0.1 || (b - pl).dot(b - a) < 0.0 {
            break;
        }

        let dist_to_border = (p - border_intersection).length();
        let mut radius = rng.get_f32(5.0..20.0).min(remaining).min(dist_to_border);
        while radius > dist_to_border {
            radius *= 0.6;
        }
        let min = dist_to_border * 0.3;
        let dist = rng.get_f32(min..(dist_to_border - radius).max(min));

        p += dir * dist;
        builder.line_to(p);

        for j in 0..5 {
            let p2 = p - dir * 1.5 * j as f32;
            let center = p2 + adv * radius;
            builder.move_to(p2);
            let arc = Arc {
                center,
                radii: vector(radius, radius),
                start_angle: (p2 - center).angle_from_x_axis(),
                sweep_angle: Angle::pi() * -sign,
                x_rotation: Angle::zero(),
            };
            let arc_steps = 100;
            for i in 1..arc_steps {
                let t = i as f32 / arc_steps as f32;
                builder.line_to(arc.sample(t));
            }
            builder.line_to(arc.sample(1.0) - dir * 2.0);
        }
        p += adv * (radius * 2.0);
        builder.move_to(p);

    sign *= -1.0;
    }

    let dir = if sign >= 0.0 { v } else { -v };
    p += dir * 50.0;
    builder.line_to(p);

    let arc = Arc {
        center: p + dir.normalize() * 5.0,
        radii: vector(5.0, 5.0),
        start_angle: Angle::zero(),
        sweep_angle: Angle::pi() * 2.0,
        x_rotation: Angle::zero(),
    };
    builder.move_to(arc.sample(0.0));
    for i in 0..100 {
        builder.line_to(arc.sample(i as f32 / 100.0));
    }

    write_path_to_stdout(&builder.build(), Orientation::Landscape);
}
