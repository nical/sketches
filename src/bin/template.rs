extern crate utils;

use utils::*;
use lyon::path::builder::*;

fn main() {
    let mut params = Params::new();

    let seed: usize = params.get_usize(42);

    let mut rng = Random::new(seed);
    let mut builder = Path::builder();

    let w = rng.get_f32(10.0..100.0);
    let h = rng.get_f32(10.0..100.0);
    builder.move_to(point(0.0, 0.0));
    builder.line_to(point(w,   0.0));
    builder.line_to(point(w,   h));
    builder.line_to(point(0.0, h));
    builder.close();

    write_path_to_stdout(&builder.build(), Orientation::Landscape);
}
