use std::time::Instant;
use std::{fmt::Display, io::Read};

use nom::branch::alt;
use nom::character::complete::{self, one_of};
use nom::combinator::iterator;
use nom::error::ErrorKind;
use nom::Parser;

type IdxType = usize;

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
struct Idx(IdxType);

impl Display for Idx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Idx: {}", self.0)
    }
}

impl Idx {
    fn as_usize(&self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
enum Direction {
    #[default]
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}
impl Direction {
    fn right(&mut self) {
        *self = match self {
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
        }
    }
    fn left(&mut self) {
        *self = match self {
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Up => Self::Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Steps {
    Right,
    Left,
    Num(u8),
}

#[derive(Debug, PartialEq, Eq)]
struct Jungle {
    field: Vec<u8>,
    size: Loc,
    current: Loc,
    dir: Direction,
    guidens: Vec<Steps>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd, Hash)]
struct Loc {
    x: u8,
    y: u8,
}

impl Jungle {
    fn from(input: &[u8]) -> Self {
        let mut field = Vec::with_capacity(10000);

        let mut size = Loc { x: 0, y: 0 };

        let mut lines = input.split(|v| *v == b'\n');

        for line in &mut lines {
            if line.is_empty() {
                break;
            }
            if size.x == 0 {
                size.x = u8::try_from(line.len()).unwrap();
            }
            let append = size.x as usize - line.len();
            field.extend_from_slice(line);
            if append != 0 {
                for _ in 0..append {
                    field.push(b' ');
                }
            }

            size.y += 1;
        }

        let idx = field.iter().position(|v| *v == b'.').unwrap();

        let input = String::from_utf8(lines.next().unwrap().to_vec()).unwrap();

        let mut it = iterator(
            input.as_str(),
            alt((
                one_of::<_, _, (&str, ErrorKind)>("RL").map(|v| match v {
                    'L' => Steps::Left,
                    'R' => Steps::Right,
                    _ => panic!("Unknown {v}"),
                }),
                complete::u8.map(|v| Steps::Num(v)),
            )),
        );
        let guidens = it.collect::<Vec<Steps>>();
        let _ = it.finish().unwrap();

        Self {
            field,
            size,
            current: Loc { x: idx as u8, y: 0 },
            dir: Direction::default(),
            guidens,
        }
    }

    fn idx(&self, pos: Loc) -> usize {
        let Loc { x, y } = pos;
        Idx(y as IdxType * self.size.x as IdxType + x as IdxType).as_usize()
    }

    fn find(&self) -> Option<Loc> {
        let Loc { mut x, mut y } = self.current;

        // print!("\tFind {x} {y} in {:?}", self.dir);
        loop {
            match self.dir {
                Direction::Right => {
                    x += 1;
                    if x == self.size.x {
                        x = 0
                    };
                }
                Direction::Down => {
                    y += 1;
                    if y == self.size.y {
                        y = 0
                    };
                }
                Direction::Left => {
                    if x == 0 {
                        x = self.size.x
                    };
                    x -= 1;
                }
                Direction::Up => {
                    if y == 0 {
                        y = self.size.y
                    };
                    y -= 1;
                }
            };

            let &c = self.field.get(self.idx(Loc { x, y })).unwrap();
            // println!("-> {x},{y} got {}", char::from(c));
            match c {
                b' ' => continue,
                b'.' => return Some(Loc { x, y }),
                b'#' => return None,
                e => panic!("Unkown {e} loc {x},{y}"),
            }
        }
    }

    pub fn walk(&mut self) -> usize {
        'lus: for step in &self.guidens {
            // println!("Step: {step:?}");
            match step {
                Steps::Right => self.dir.right(),
                Steps::Left => self.dir.left(),
                Steps::Num(num) => {
                    for _n in 0..*num {
                        let pos = self.find();
                        if let Some(loc) = pos {
                            // println!("{n}: {:?} -> {loc:?}", self.current);
                            self.current = loc;
                        } else {
                            continue 'lus;
                        }
                    }
                }
            }
        }

        (self.current.y as usize + 1) * 1000 + (self.current.x as usize + 1) * 4 + self.dir as usize
    }
}

fn main() {
    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = Vec::<u8>::with_capacity(1_000_000);
    f.read_to_end(&mut input).unwrap();

    let mut jungle = Jungle::from(&input);

    let start = Instant::now();
    let password = jungle.walk();
    let end = start.elapsed();

    println!("Part1: Password: {password} in {} uS", end.as_micros());
}

#[cfg(test)]

mod tests {
    const INPUT: &[u8] = include_bytes!("../input/test.txt");

    use super::*;

    #[test]
    fn test_example1() {
        let mut jungle = Jungle::from(INPUT);

        println!("jungle {:?}", jungle.current);
        println!("jungle {:?}", jungle.guidens);

        assert_eq!(jungle.walk(), 6032);
    }
}
