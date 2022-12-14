use std::{io::Read, time::Instant};

mod pos {
    pub const EMPTY: u8 = b'.';
    pub const WALL: u8 = b'#';
    pub const SAND: u8 = b'O';
    pub const START: u8 = b'+';
}

const FIELD_X: usize = 500 * 2;
const FIELD_Y: usize = 170;
struct Sim {
    buf: [u8; FIELD_X * FIELD_Y],
}

impl Sim {
    fn new() -> Self {
        Self {
            buf: [pos::EMPTY; FIELD_X * FIELD_Y],
        }
    }

    fn draw(&mut self) {
        // self.buf[Vector(500, 0).to_addr()] = pos::START;

        for (y, line) in self.buf.chunks(FIELD_X).enumerate() {
            print!("{y} ");
            print_input(&line[500 - 40..500 + 40]);
            if y == 200 {
                break;
            }
        }
    }

    // 498,4 -> 498,6 -> 496,6
    fn draw_vectors(&mut self, vecs: Vec<Vector>) {
        let mut vecs = vecs.iter();
        let mut wall = vecs.next().unwrap().to_owned();
        let mut addr = wall.to_addr();
        self.buf[addr] = pos::WALL;

        for p_next in vecs {
            let (step, cnt) = p_next.step_addr(&wall);
            for _ in 0..cnt {
                if step.is_negative() {
                    addr -= step.abs() as usize;
                } else {
                    addr += step.to_owned() as usize;
                }
                self.buf[addr] = pos::WALL;
                wall = p_next.clone();
            }
        }
    }

    fn sim(&mut self) -> bool {
        let mut addr = Vector(500, 0).to_addr();

        let start = self.buf[addr];
        if start != pos::EMPTY { return false };

        let mut last_valid;

        loop {
            last_valid = addr;
            addr += FIELD_X;
            if addr >= self.buf.len() {
                return false;
            }
            if self.buf[addr] == pos::EMPTY {
                continue;
            }
            addr -= 1;
            if self.buf[addr] == pos::EMPTY {
                continue;
            }
            addr += 2;
            if self.buf[addr] == pos::EMPTY {
                continue;
            }

            break;
        }
        self.buf[last_valid] = pos::SAND;

        true
    }
}


struct Timer(Instant);

impl Timer {
    fn new() -> Self {
        Self (std::time::Instant::now())
    }
    fn update(&mut self, data: &str) {
        let elapsed = self.0.elapsed();
        println!("{data}: {} uS", elapsed.as_micros());
        self.0 = std::time::Instant::now();
    }
}

fn main() {
    let mut total =Timer::new();
    let mut timer = Timer::new();

    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = Vec::<u8>::with_capacity(1_000_000);

    f.read_to_end(&mut input).unwrap();

    timer.update("Load data");

    let points = input.split(|v| *v == b'\n');

    let mut sim = Sim::new();
    let mut max: u16 = 0;

    for pi in points {
        let vecs = Vector::from(pi);
        let max_y = vecs.iter().map(|v| v.1).max().unwrap();
        max = max.max(max_y);
        sim.draw_vectors(vecs);
    }

    timer.update("Setup board");

    let mut cnt = 0;
    loop {
        if !sim.sim() {
            break;
        }
        cnt += 1;
    }

    timer.update("Part1");

    println!("num: {cnt}");

    max += 2;
    let vecs = vec![Vector(0, max), Vector(999, max)];
    sim.draw_vectors(vecs);

    // Part2
    println!("max: {max}");

    loop {
        if !sim.sim() {
            break;
        }
        cnt += 1;
    }

    timer.update("Part2");

    // sim.draw();

    println!("Part2: num: {cnt}");

    total.update("Total");
}

#[derive(Debug, PartialEq, Eq)]
struct Vector(u16, u16);

impl Vector {
    fn from(input: &[u8]) -> Vec<Self> {
        let mut points = Vec::<Self>::with_capacity(10);

        for p in input.split(|v| *v == b' ') {
            if p == b"->" {
                continue;
            }

            let mut value = p.split(|v| *v == b',');
            points.push(Self(
                slice_to_number(value.next().unwrap()),
                slice_to_number(value.next().unwrap()),
            ));
        }
        points
    }

    fn to_addr(&self) -> usize {
        let addr = usize::from(self.1) * FIELD_X + usize::from(self.0);

        addr
    }

    // rhs 2,0 self 1,0
    fn step_addr(&self, from: &Vector) -> (i16, i16) {
        let x = i16::try_from(self.0).unwrap() - i16::try_from(from.0).unwrap();
        let y = i16::try_from(self.1).unwrap() - i16::try_from(from.1).unwrap();

        let ret = if y == 0 {
            (x.max(-1).min(1), x.abs())
        } else {
            (y.max(-1).min(1) * i16::try_from(FIELD_X).unwrap(), y.abs())
        };
        // println!("self: {:?} to: {:?} x:{x}, y{y} ret {} {}", self, from, ret.0, ret.1);
        ret
    }
}

fn print_input(input: &[u8]) {
    println!("input: {}", core::str::from_utf8(input).unwrap());
}

fn slice_to_number(input: &[u8]) -> u16 {
    // print_input(input);
    input.iter().map(|v| *v - b'0').fold(0, |sum, x| {
        sum.checked_mul(10)
            .unwrap()
            .checked_add(u16::from(x))
            .unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::{Sim, Vector};

    const INPUT: &[u8] = b"498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_points() {
        let mut points = INPUT.split(|v| *v == b'\n');

        let pi = points.next().unwrap();
        let vecs = Vector::from(pi);
        assert_eq!(vecs, vec![Vector(498, 4), Vector(498, 6), Vector(496, 6)]);

        let pi = points.next().unwrap();
        let vecs = Vector::from(pi);
        assert_eq!(
            vecs,
            vec![
                Vector(503, 4),
                Vector(502, 4),
                Vector(502, 9),
                Vector(494, 9)
            ]
        );

        assert_eq!(points.next(), None);
    }

    #[test]
    fn test_point_to_addr() {
        assert_eq!(Vector(0, 0).to_addr(), 0);
        assert_eq!(Vector(0, 1).to_addr(), 1000);
        assert_eq!(Vector(500, 1).to_addr(), 1500);
    }

    #[test]
    fn test_point_step() {
        assert_eq!(Vector(1, 0).step_addr(&Vector(0, 0)), (1, 1));
        assert_eq!(Vector(1, 0).step_addr(&Vector(2, 0)), (-1, 1));
        assert_eq!(Vector(0, 0).step_addr(&Vector(0, 1)), (-1000, 1));
        assert_eq!(Vector(0, 1).step_addr(&Vector(0, 0)), (1000, 1));
        assert_eq!(Vector(1, 0).step_addr(&Vector(3, 0)), (-1, 2));
        assert_eq!(Vector(0, 3).step_addr(&Vector(0, 0)), (1000, 3));
    }

    #[test]
    fn test_draw_example() {
        let points = INPUT.split(|v| *v == b'\n');

        let mut sim = Sim::new();

        for pi in points {
            let vecs = Vector::from(pi);
            sim.draw_vectors(vecs);
        }

        sim.draw();

        for c in 0..26 {
            if !sim.sim() {
                assert_eq!(c, 24);
                break;
            }
        }

        sim.draw();

        // assert!(false);
    }

    #[test]
    fn test_draw_example_part2() {
        let points = INPUT.split(|v| *v == b'\n');

        let mut sim = Sim::new();

        let mut max: u16 = 0;

        for pi in points {
            let vecs = Vector::from(pi);
            let max_y = vecs.iter().map(|v| v.1).max().unwrap();
            max = max.max(max_y);
            sim.draw_vectors(vecs);
        }

        assert_eq!(max, 9);

        max += 2;

        let vecs = vec![Vector(0, max), Vector(999, max)];
        sim.draw_vectors(vecs);

        sim.draw();

        for c in 0..100 {
            if !sim.sim() {
                assert_eq!(c, 93);
                break;
            }
        }

        sim.draw();

        // assert!(false);
    }
}
