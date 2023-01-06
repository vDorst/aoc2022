use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io::Read, ops::Add, time::Instant,
};

use itertools::{MinMaxResult::{self, MinMax}, Itertools};

fn part1(input: &[u8]) -> usize {
    let mut dec = Grove::new(input);
    
    dec.process(Some(3)).0
}

fn part2(input: &[u8]) -> usize {
    let mut dec = Grove::new(input);
    dec.process(None).1
}

fn main() {
    let start_begin = Instant::now();

    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = Vec::with_capacity(1_000_000);
    f.read_to_end(&mut input).unwrap();

    let start = Instant::now();
    let answer = part1(&input);
    println!("Part1: ground: {} in {} uS", answer, start.elapsed().as_micros());

    let start = Instant::now();
    let answer = part2(&input);
    println!("Part2: rounds: {} in {} uS", answer, start.elapsed().as_micros());

    println!("Done in {} mS", start_begin.elapsed().as_millis());
}

type Idx = i16;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Loc(Idx, Idx);

impl Add for Loc {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Loc(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

type Moves = HashMap<Loc, Option<Loc>>;

struct Grove {
    elves: HashSet<Loc>,
    moves: Moves,
}

const CHECKS: [(u8, Loc); 4] = [ ( 0b1110_0000, Loc(0, -1)), (0b0000_0111, Loc(0, 1)), ( 0b1001_0100, Loc(-1, 0)), (0b0010_1001, Loc(1, 0)) ];

fn push_move(moves: &mut Moves, new_pos: Loc, cur_pos: Loc) {
    let val = if moves.contains_key(&new_pos) {
        None
    } else {
        Some(cur_pos)
    };
    // println!("Insert: {new_pos} from {val:?}");
    moves.insert(new_pos, val);
}

impl Grove {
    fn new(input: &[u8]) -> Self {
        let mut size = Loc(0, 0);

        let mut elves = HashSet::with_capacity(5000);

        for line in input.split(|v| *v == b'\n') {
            size.0 = line.len() as Idx;

            let y = size.1;
            for (c, x) in line.iter().zip(0 as Idx..) {
                if *c == b'#' {
                    elves.insert(Loc(x, y));
                }
            }

            size.1 += 1;
        }

        Self {
            elves,
            moves: HashMap::with_capacity(5000),
        }
    }

    fn process(&mut self, cycles: Option<usize>) -> (usize, usize) {

        let mut r = 0;

        loop {
            let moves = &mut self.moves;
            // println!("Round: {}", r + 1);
            moves.clear();
            for curr_loc in self.elves.iter() {
                let mut others: u8 = 0;
                //let mut wall: u8 = 0;
                for ys in -1..=1 {
                    for xs in -1..=1 {
                        let search_pos = Loc(xs, ys) + *curr_loc;
                        if search_pos == *curr_loc {
                            continue;
                        }
                        others <<= 1;
                        // <<= 1;
                        // let x_valid = curr_loc.0 + xs >= 0 && curr_loc.0 + xs < self.size.0;
                        // let y_valid = curr_loc.1 + ys >= 0 && curr_loc.1 + ys < self.size.1;
                        // if x_valid && y_valid {
                            if self.elves.contains(&search_pos) {
                                others |= 0x01;
                            }
                        // } else {
                        //     wall |= 0x01;
                        // }
                    }
                }
                // println!("{curr_loc} - {others:08b}");

                if others != 0 {
                    let mut checks_iter = CHECKS.iter().cycle().skip(r);

                    for _ in 0..CHECKS.len() {
                        let (mask, pos) = checks_iter.next().unwrap();
                        if others & mask == 0 {
                            let new_pos = *curr_loc + *pos;
                            push_move(moves, new_pos, *curr_loc);
                            break;
                        }
                    }
                }
            }

            r += 1;

            if moves.is_empty() {
                break;
            }
            for (&new, curr) in moves.iter() {
                if let Some(pos) = curr {
                    self.elves.remove(pos);
                    self.elves.insert(new);
                }
            }
            // self.show_map();
            // dbg!(self.elves.len());


            if let Some(rnum) = cycles {
                if rnum == r {
                    break;
                }
            }
        }

        let  (x1, x2, y1, y2) = self.get_rectangle();

        ((x2.abs_diff(x1) + 1) as usize * (y2.abs_diff(y1) + 1) as usize - self.elves.len(), r)
    }

    fn get_rectangle(&self) -> (Idx, Idx, Idx, Idx) {
        let xspan = self.elves.iter().map(|p| p.0).minmax();
        let yspan = self.elves.iter().map(|p| p.1).minmax();

        let (MinMax(x1,x2), MinMax(y1, y2)) = (xspan, yspan) else { panic!() };

        (x1, x2, y1, y2)
    }

    fn show_map(&self) {
        let  (x1, x2, y1, y2) = self.get_rectangle();

        for y in y1.min(0)..=y2.max(5) {
            print!("{y:>03}: ");
            for x in x1.min(0)..=x2.max(5) {
                let c = if self.elves.contains(&Loc(x, y)) {
                    '#'
                } else {
                    '.'
                };
                print!("{c}");
            }
            println!("");
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_SMALL_START: &[u8] = b".....
..##.
..#..
.....
..##.
.....";

const INPUT: &[u8] = b"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    #[test]
    fn test_example_small() {
        let mut dec = Grove::new(INPUT_SMALL_START);

        dec.process(Some(3));

    }

    #[test]
    fn test_example_part1() {
        let mut dec = Grove::new(INPUT);

        assert_eq!(dec.process(Some(10)), (110, 10));

    }

    #[test]
    fn test_example_part2() {
        let mut dec = Grove::new(INPUT);

        assert_eq!(dec.process(None).1, 20);

    }
}
