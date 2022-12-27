use std::{fmt::Display, io::Read};

fn part1(input: &[u8]) -> usize {
    let steps = input
        .split(|v| *v == b'\n')
        .map(|v| {
            let step: i8 = core::str::from_utf8(&v[2..]).unwrap().parse().unwrap();

            let m = match v[0] {
                b'U' => EMove::Up,
                b'L' => EMove::Left,
                b'R' => EMove::Right,
                b'D' => EMove::Down,
                e => panic!("{e:X?}"),
            };
            Move(m, step)
        })
        .collect::<Vec<Move>>();

    let mut sim = Sim::new(2);

    for m in &steps {
        sim.step(m);
    }

    // println!("Max: x{} y{} l{}", sim.m_x, sim.m_y, sim.list.len());

    sim.list.len()
}

fn part2(input: &[u8]) -> usize {
    let steps = input
        .split(|v| *v == b'\n')
        .map(|v| {
            let step: i8 = core::str::from_utf8(&v[2..]).unwrap().parse().unwrap();

            let m = match v[0] {
                b'U' => EMove::Up,
                b'L' => EMove::Left,
                b'R' => EMove::Right,
                b'D' => EMove::Down,
                e => panic!("{e:X?}"),
            };
            Move(m, step)
        })
        .collect::<Vec<Move>>();

    let mut sim = Sim::new(10);

    for m in &steps {
        sim.step(m);
    }

    // println!("Max: x{} y{} l{}", sim.m_x, sim.m_y, sim.list.len());

    sim.list.len()
}

// fn part2(input: &str) -> u32 {
//     let tree = build_tree(input);

//     let disk_usage = tree[0].size;

//     let current_free = DISK_SPACE - disk_usage;

//     let size_to_delete = DISK_SPACE_NEEDED - current_free;

//     let dir_size: u32 = tree
//         .iter()
//         .filter(|e| e.item == Item::Dir && e.size >= size_to_delete)
//         .map(|e| e.size)
//         .min().unwrap();

//     dir_size
// }

fn main() {
    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = Vec::<u8>::with_capacity(1_000_000);
    f.read_to_end(&mut input).unwrap();

    let answer = part1(&input);
    println!("Part1: sum: {}", answer);
    assert_eq!(answer, 6087);
    // println!("Part2: sum: {}", part2(&input));

    let answer = part2(&input);
    println!("Part2: sum: {}", answer);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum EMove {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Move(EMove, i8);

type Pos = i32;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Loc {
    x: Pos,
    y: Pos,
}

impl Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.x, self.y)
    }
}

impl Loc {
    fn up(&mut self) {
        self.y += 1;
    }

    fn down(&mut self) {
        self.y -= 1;
    }

    fn left(&mut self) {
        self.x -= 1;
    }
    fn right(&mut self) {
        self.x += 1;
    }

    fn new() -> Loc {
        Loc { x: 0, y: 0 }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Sim {
    rope: Vec<Loc>,
    list: Vec<Loc>,
}
impl Sim {
    fn new(size: usize) -> Self {
        let mut sim = Self {
            rope: vec![Loc::new(); size],
            list: Vec::with_capacity(100),
        };
        sim.list.push(Loc::new());
        sim
    }

    fn step(&mut self, m: &Move) {
        for _ in 0..m.1 {
            let mut points = self.rope.iter_mut();

            let mut prev = {
                let head = points.next().unwrap();

                match m.0 {
                    EMove::Up => {
                        head.up();
                    }
                    EMove::Down => {
                        head.down();
                    }
                    EMove::Left => {
                        head.left();
                    }
                    EMove::Right => {
                        head.right();
                    }
                };
                head.clone()
            };

            for curr in points {
                let diff = prev.x.abs_diff(curr.x) + prev.y.abs_diff(curr.y);
                if diff < 2 {
                    break;
                }

                if prev.x.abs_diff(curr.x) == 1 && prev.y.abs_diff(curr.y) == 1 {
                    break;
                }

                let diff = prev.x - curr.x;
                if diff < 0 {
                    curr.left();
                }
                if diff > 0 {
                    curr.right();
                }

                let diff = prev.y - curr.y;
                if diff < 0 {
                    curr.down();
                }
                if diff > 0 {
                    curr.up();
                }

                prev = curr.clone();
            }

            // println!(
            //     "h {}x{}; t {}x{} d{diff} m{m:?} H{:?}",
            //     h.x, h.y, t.x, t.y, h
            // );

            let last = self.rope.last().unwrap();
            if !self.list.iter().any(|v| v == last) {
                self.list.push(last.clone());
            }
        }
    }

    fn draw(&self) {
        for y in 0..10 {
            for x in 0..10 {
                if self.list.iter().any(|v| *v == Loc { x, y }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    fn draw_map(&self) {
        println!("n: 0123456789");
        for y in (0..9).rev() {
            print!("{y}: ");
            for x in 0..10 {
                if let Some((idx, _v)) = self
                    .rope
                    .iter()
                    .enumerate()
                    .find(|(_idx, v)| **v == Loc { x, y })
                {
                    let c = match idx {
                        0 => b'H',
                        e => e as u8 + b'0',
                    };
                    print!("{}", char::from(c));
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{EMove, Loc, Move, Sim};

    const INPUT: &[u8] = b"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_example() {
        let steps = INPUT
            .split(|v| *v == b'\n')
            .map(|v| {
                let step: i8 = core::str::from_utf8(&v[2..]).unwrap().parse().unwrap();
                let m = match v[0] {
                    b'U' => EMove::Up,
                    b'L' => EMove::Left,
                    b'R' => EMove::Right,
                    b'D' => EMove::Down,
                    e => panic!("{e:X?}"),
                };
                Move(m, step)
            })
            .collect::<Vec<Move>>();

        assert_eq!(steps.len(), 8);
        assert_eq!(steps[0], Move(EMove::Right, 4));

        let mut sim = Sim::new(2);

        let mut step = steps.iter();

        let m = step.next().unwrap();
        sim.step(m);
        assert_eq!(sim.rope[1], Loc { x: 3, y: 0 });
        let m = step.next().unwrap();
        sim.step(m);
        assert_eq!(sim.rope[1], Loc { x: 4, y: 3 });
        let m = step.next().unwrap();
        sim.step(m);
        assert_eq!(sim.rope[1], Loc { x: 2, y: 4 });
        let m = step.next().unwrap();
        sim.step(m);
        assert_eq!(sim.rope[1], Loc { x: 2, y: 4 });
        let m = step.next().unwrap();
        sim.step(m);
        assert_eq!(sim.rope[1], Loc { x: 4, y: 3 });
        let m = step.next().unwrap();
        sim.step(m);
        assert_eq!(sim.rope[1], Loc { x: 4, y: 3 });

        let m = step.next().unwrap();
        sim.step(m);
        assert_eq!(sim.rope[1], Loc { x: 1, y: 2 });
        let m = step.next().unwrap();
        sim.step(m);
        assert_eq!(sim.rope[1], Loc { x: 1, y: 2 });

        assert_eq!(step.next(), None);
        sim.draw();

        assert_eq!(sim.list.len(), 13);
    }

    #[test]
    fn test_example1() {
        let mut sim = Sim::new(2);

        assert_eq!(sim.rope[0], Loc { x: 0, y: 0 });
        sim.step(&Move(EMove::Up, 1));
        assert_eq!(sim.rope[0], Loc { x: 0, y: 1 });

        sim.step(&Move(EMove::Right, 5));

        sim.step(&Move(EMove::Down, 2));

        assert_eq!(sim.rope[0], Loc { x: 5, y: -1 });

        assert_eq!(Loc::new(), Loc { x: 0, y: 0 });
    }

    #[test]
    fn test_example2() {
        let mut sim = Sim::new(10);

        assert_eq!(sim.rope[0], Loc { x: 0, y: 0 });
        sim.step(&Move(EMove::Right, 5));
        sim.step(&Move(EMove::Up, 8));
        sim.draw_map();

        assert_eq!(*sim.rope.last().unwrap(), Loc { x: 0, y: 0 });

        sim.step(&Move(EMove::Left, 8));
        sim.draw_map();

        assert_eq!(*sim.rope.last().unwrap(), Loc { x: 1, y: 3 });

        sim.step(&Move(EMove::Down, 3));
        sim.step(&Move(EMove::Right, 17));
        sim.draw_map();

        assert_eq!(*sim.rope.last().unwrap(), Loc { x: 5, y: 5 });
    }
}
