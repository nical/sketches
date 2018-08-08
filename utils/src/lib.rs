pub extern crate lyon;
pub extern crate image;
pub extern crate rand;

use image::*;
use std::path::Path as FilePath;

use std::ops::Range;
use std::mem::transmute;
use rand::rngs::StdRng;
use rand::*;

pub use std::f32::consts::PI;
pub use lyon::path::default::{Path, Builder};
pub use lyon::geom::math::*;
pub use lyon::geom::*;

use lyon::geom::euclid::{ size2, Rect };
use lyon::svg::path_utils::PathSerializer;
use lyon::algorithms::fit::*;
use lyon::path::builder::*;

pub struct Random {
    pub rng: StdRng,
}

impl Random {
    pub fn new(seed: usize) -> Self {
        Random {
            rng: StdRng::from_seed(unsafe { transmute([seed; 4]) })
        }
    }

    pub fn get_f32(&mut self, range: Range<f32>) -> f32 {
        assert!(range.end >= range.start);
        let mut r = (self.rng.next_u32() as f32) / (std::u32::MAX as f32);
        r = r * (range.end - range.start) + range.start;
        r
    }

    pub fn get_u32(&mut self, range: Range<u32>) -> u32 {
        assert!(range.end >= range.start);
        range.start + self.rng.next_u32() % (range.end - range.start)
    }

    pub fn get_bool(&mut self) -> bool {
        self.get_u32(0..11) % 2 == 0
    }
}

pub fn load_image(path: &str) -> ImageResult<DynamicImage> {
    image::open(&FilePath::new(&path))
}

pub fn step(x: f32, range: Range<f32>) -> f32 {
    if x < range.start {
        return 0.0;
    }
    if x > range.end {
        return 1.0;
    }

    (x - range.start) / (range.end - range.start)
}

pub trait Clamp : Sized {
    fn clamp(&self, range: Range<Self>) -> Self;
}

impl Clamp for f32 {
    fn clamp(&self, range: Range<f32>) -> Self {
        if *self < range.start {
            return range.start;
        }
        if *self > range.end {
            return range.end;
        }

        return *self
    }
}

impl Clamp for f64 {
    fn clamp(&self, range: Range<f64>) -> Self {
        if *self < range.start {
            return range.start;
        }
        if *self > range.end {
            return range.end;
        }

        return *self
    }
}

impl Clamp for u32 {
    fn clamp(&self, range: Range<u32>) -> Self {
        if *self < range.start {
            return range.start;
        }
        if *self > range.end {
            return range.end;
        }

        return *self
    }
}

impl Clamp for i32 {
    fn clamp(&self, range: Range<i32>) -> Self {
        if *self < range.start {
            return range.start;
        }
        if *self > range.end {
            return range.end;
        }

        return *self
    }
}

impl Clamp for usize {
    fn clamp(&self, range: Range<usize>) -> Self {
        if *self < range.start {
            return range.start;
        }
        if *self > range.end {
            return range.end;
        }

        return *self
    }
}

impl Clamp for isize {
    fn clamp(&self, range: Range<isize>) -> Self {
        if *self < range.start {
            return range.start;
        }
        if *self > range.end {
            return range.end;
        }

        return *self
    }
}

impl Clamp for u16 {
    fn clamp(&self, range: Range<u16>) -> Self {
        if *self < range.start {
            return range.start;
        }
        if *self > range.end {
            return range.end;
        }

        return *self
    }
}

impl Clamp for i16 {
    fn clamp(&self, range: Range<i16>) -> Self {
        if *self < range.start {
            return range.start;
        }
        if *self > range.end {
            return range.end;
        }

        return *self
    }
}

use std::env;

pub struct Params {
    nth: usize,
}

impl Params {
    pub fn new() -> Self {
        Params {
            nth: 1,
        }
    }

    pub fn get_f32(&mut self, default: f32) -> f32 {
        let result: f32 = if let Some(s) = env::args().nth(self.nth) {
            s.parse::<f32>().unwrap_or(default)
        } else {
            default
        };

        self.nth += 1;

        result
    }

    pub fn get_i32(&mut self, default: i32) -> i32 {
        let result = if let Some(s) = env::args().nth(self.nth) {
            s.parse().unwrap_or(default)
        } else {
            default
        };

        self.nth += 1;

        result
    }

    pub fn get_usize(&mut self, default: usize) -> usize {
        let result = if let Some(s) = env::args().nth(self.nth) {
            s.parse().unwrap_or(default)
        } else {
            default
        };

        self.nth += 1;

        result
    }

    pub fn get_string(&mut self, default: &str) -> String {
        let result = if let Some(s) = env::args().nth(self.nth) {
            s.to_string()
        } else {
            default.to_string()
        };

        self.nth += 1;

        result
    }
}

pub enum Orientation {
    Portrait,
    Landscape,
}

pub fn write_path_to_stdout(path: &Path, orientation: Orientation) {
    let scaled_path = fit_path(
        path,
        &Rect {
            origin: point(0.0, 0.0),
            size: match orientation {
                Orientation::Landscape => size2(297.0, 210.0),
                Orientation::Portrait => size2(210.0, 297.0),
            }
        }.inflate(-5.0, -5.0),
        FitStyle::Min,
    );

    let mut svg_out = PathSerializer::new();

    for event in scaled_path.iter() {
        svg_out.path_event(event);
    }

    println!("{}", svg_out.build());
}
