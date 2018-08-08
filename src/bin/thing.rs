extern crate utils;

use utils::*;
use lyon::path::builder::*;

const LEFT: u32 = 1 << 0;
const RIGHT: u32 = 1 << 1;
const UP: u32 = 1 << 2;
const DOWN: u32 = 1 << 3;

struct Cells {
    cells: Vec<u32>,
    w: usize,
    h: usize,
}

impl Cells {
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.w + x
    }
}

fn draw_cell(cells: &Cells, x: usize, y: usize, builder: &mut Builder) {

}


fn main() {
    let mut params = Params::new();

    let seed: usize = params.get_usize(42);
    let w = params.get_usize(20);
    let h = params.get_usize(20);

    let mut rng = Random::new(seed);

    let mut board = Cells {
        cells: vec![0; w * h],
        w,
        h,
    };

    let mut builder = Path::builder();

    for y in 0..h {
        for x in 0..w {
            let idx = board.index(x, y);
            let mut cell = 0;
            if rng.get_u32(0..4) == 0 && x > 0 { cell |= LEFT; }
            if rng.get_u32(0..4) == 0 && x < w { cell |= RIGHT; }
            if rng.get_u32(0..4) == 0 && y > 0 { cell |= UP; }
            if rng.get_u32(0..4) == 0 && y < h { cell |= DOWN; }
            board.cells[idx as usize] = cell;
        }
    }

    for y in 1..(h-1) {
        for x in 1..(w-1) {
            let idx = board.index(x, y);
            let up_idx = board.index(x, y-1);
            let down_idx = board.index(x, y+1);
            let left_idx = board.index(x-1, y);
            let right_idx = board.index(x+1, y);
            if board.cells[idx] & UP != 0 { board.cells[up_idx] |= DOWN; }
            if board.cells[idx] & DOWN != 0 { board.cells[down_idx] |= UP; }
            if board.cells[idx] & LEFT != 0 { board.cells[left_idx] |= RIGHT; }
            if board.cells[idx] & RIGHT != 0 { board.cells[right_idx] |= LEFT; }
        }
    }

    for y in 0..h {
        for x in 0..w {
            let mut cell = board.cells[board.index(x, y)];
            let origin = point(
                x as f32 * 10.0,
                y as f32 * 10.0,
            );
            if cell & DOWN != 0 && cell & RIGHT != 0 {
                //builder.move_to(origin + vector(5.0, 10.0));
                //builder.line_to(origin + vector(10.0, 5.0));
                cell &= !DOWN;
                cell &= !RIGHT;
                arc(origin + vector(10.0, 10.0), PI, &mut builder);
            }
            if cell & UP != 0 && cell & LEFT != 0 {
                //builder.move_to(origin + vector(0.0, 5.0));
                //builder.line_to(origin + vector(5.0, 0.0));
                cell &= !UP;
                cell &= !LEFT;
                arc(origin, 0.0, &mut builder);
            }

            if cell & UP != 0 && cell & DOWN != 0 {
                builder.move_to(origin + vector(5.0, 0.0));
                builder.line_to(origin + vector(5.0, 10.0));
                cell &= !UP;
                cell &= !DOWN;
            }

            if cell & LEFT != 0 && cell & RIGHT != 0 {
                builder.move_to(origin + vector(0.0, 5.0));
                builder.line_to(origin + vector(10.0, 5.0));
                cell &= !LEFT;
                cell &= !RIGHT;
            }

            if cell == LEFT {
                builder.move_to(origin + vector(0.0, 5.0));
                builder.line_to(origin + vector(2.0, 5.0));
            }
            if cell == RIGHT {
                builder.move_to(origin + vector(10.0, 5.0));
                builder.line_to(origin + vector(8.0, 5.0));
            }
            if cell == UP {
                builder.move_to(origin + vector(5.0, 0.0));
                builder.line_to(origin + vector(5.0, 2.0));
            }
            if cell == DOWN {
                builder.move_to(origin + vector(5.0, 0.0));
                builder.line_to(origin + vector(5.0, 2.0));
            }
        }
    }

    write_path_to_stdout(&builder.build(), Orientation::Landscape);
}

fn arc(center: Point, start: f32, builder: &mut Builder) {
    let arc = Arc {
        center,
        radii: vector(5.0, 5.0),
        start_angle: Angle::radians(start),
        sweep_angle: Angle::pi() / 2.0,
        x_rotation: Angle::zero(),
    };

    builder.move_to(arc.sample(0.0));
    for i in 0..101 {
        builder.line_to(arc.sample(i as f32 / 100.0));
    }
}
