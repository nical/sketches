type PatternId = usize;

struct Pattern {
    left: u64,
    right: u64,
    up: u64,
    down: u64,
    bit: u64,
}

struct Model {
    patterns: Vec<Pattern>,
}

struct Wave {
    cells: Vec<u64>,
    width: usize,
    height: usize,
}

const SKY_BIT: u64 = 1 << 0;
const GROUND_BIT: u64 = 2 << 0;
const GROUND_SKYP_UP: u64 = 2 << 0;

fn main() {
    let model = Model {
        patterns: vec![
            Pattern {
                bit: SKY_BIT,
                left: 0,
                right: 0,
                up: 0,
                down: 0,
            },
            Pattern {
                bit: GROUND_BIT,
                left: 0,
                right: 0,
                up: 0,
                down: 0,
            },
        ],
    };
}

