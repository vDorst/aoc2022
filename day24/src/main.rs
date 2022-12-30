use std::time::Instant;
use std::io::Read;

use itertools::Itertools;
use num::Integer;
use petgraph::algo::dijkstra;
use petgraph::prelude::{DiGraphMap, GraphMap};
use petgraph::Directed;

type Node = (Loc, u16);
type Minutes = u16;

fn part1(graph: &GraphMap<Node, Minutes, Directed>, sim: &Sim) -> Minutes {
    let start_begin = Instant::now();

    let end_point = Loc {
        x: sim.size.x - 2,
        y: sim.size.y - 1,
    };

    let start_point = (Loc { x: 1, y: 0 }, 0_u16);

    let ans = find_path(&graph, start_point, end_point);

    println!("Total: {ans:?} in {} mS", start_begin.elapsed().as_millis());

    ans.unwrap().1
}

fn part2(graph: &GraphMap<Node, Minutes, Directed>, sim: &Sim) -> Vec<Minutes> {
    let mut total = Vec::<Minutes>::with_capacity(3);
    let start_begin = Instant::now();

    let end_point = Loc {
        x: sim.size.x - 2,
        y: sim.size.y - 1,
    };

    let start_point = (Loc { x: 1, y: 0 }, 0_u16);

    let ans = find_path(&graph, start_point, end_point);

    total.push(ans.unwrap().1);

    // Go Back
    let start_point: Node = ans.unwrap().0;

    let end_point = Loc { x: 1, y: 0 };

    let ans = find_path(&graph, start_point, end_point);

    total.push(ans.unwrap().1);

    // Go Back Again
    let start_point: Node = ans.unwrap().0;

    let end_point = Loc {
        x: sim.size.x - 2,
        y: sim.size.y - 1,
    };

    let ans = find_path(&graph, start_point, end_point);

    total.push(ans.unwrap().1);

    let sum: u16 = total.iter().sum();
    println!("Total: {total:?} = {sum} in {} mS", start_begin.elapsed().as_millis());

    total
}

fn main() {
    let start_begin = Instant::now();

    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = Vec::with_capacity(1_000_000);
    f.read_to_end(&mut input).unwrap();

    let mut sim = Sim::new(&input);

    let end = start_begin.elapsed();
    println!("Loaded map: {} uS", end.as_micros());

    let start = Instant::now();
    let edges = sim.sim();
    let end = start.elapsed();
    println!("generate edges: {} uS", end.as_micros());

    let start = Instant::now();
    let graph: GraphMap<Node, Minutes, Directed> = DiGraphMap::from_edges(&edges);
    let end = start.elapsed();
    println!("create graph: {} uS", end.as_micros());

    let answer = part1(&graph, &sim);
    println!("Part1: {}", answer);

    let answer = part2(&graph, &sim);
    let sum: u16 = answer.iter().sum();
    println!("Part2: sum: {answer:?} = {sum}");
}

const INPUT: &[u8] = b"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd, Hash)]
struct Loc {
    x: u8,
    y: u8,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Wall,
}

impl Direction {
    fn as_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Wall => '#',
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Blizzard {
    pos: Loc,
    dir: Direction,
}

#[derive(Debug, PartialEq, Eq)]
struct Sim {
    blizzards: Vec<Blizzard>,
    pos: Loc,
    size: Loc,
    minutes: Minutes,
}

impl Sim {
    fn new(input: &[u8]) -> Self {
        let mut sim = Sim {
            pos: Loc { x: 1, y: 0 },
            blizzards: Vec::with_capacity(37 * 103),
            size: Loc { x: 0, y: 0 },
            minutes: 0,
        };

        for (y, line) in (0_u8..).zip(input.split(|&v| v == b'\n')) {
            for (x, c) in (0_u8..).zip(line) {
                let dir: Direction = match c {
                    b'.' => continue,
                    b'^' => Direction::Up,
                    b'v' => Direction::Down,
                    b'<' => Direction::Left,
                    b'>' => Direction::Right,
                    b'#' => Direction::Wall,
                    e => panic!("Unknown {e:x} {}", char::from(*e)),
                };
                sim.blizzards.push(Blizzard {
                    pos: Loc { x, y },
                    dir,
                });
            }
        }

        let mut loc = sim.blizzards.last().unwrap().pos;
        loc.x += 1;
        loc.y += 1;
        sim.size = loc;

        println!(
            "blizzards len {} size = {}",
            sim.blizzards.len(),
            sim.blizzards.len() * std::mem::size_of::<Blizzard>()
        );

        sim
    }

    fn blizzards_next(&mut self) {
        for blis in self.blizzards.iter_mut() {
            match blis.dir {
                Direction::Up => {
                    blis.pos.y -= 1;
                    if blis.pos.y == 0 {
                        blis.pos.y = self.size.y - 2
                    }
                }
                Direction::Down => {
                    blis.pos.y += 1;
                    if blis.pos.y == self.size.y - 1 {
                        blis.pos.y = 1
                    }
                }
                Direction::Left => {
                    blis.pos.x -= 1;
                    if blis.pos.x == 0 {
                        blis.pos.x = self.size.x - 2
                    }
                }
                Direction::Right => {
                    blis.pos.x += 1;
                    if blis.pos.x == self.size.x - 1 {
                        blis.pos.x = 1
                    }
                }
                Direction::Wall => (),
            }
        }
    }

    fn get_empty_spots(&self) -> Vec<Loc> {
        let empty_spots: Vec<Loc> = (0..(self.size.y))
            .cartesian_product(0..(self.size.x))
            .flat_map(|(y, x)| {
                let search_spot = Loc { x, y };
                if !self.blizzards.iter().any(|b| b.pos == search_spot) {
                    Some(search_spot)
                } else {
                    None
                }
            })
            .collect();
        // dbg!(empty_spots.len());
        empty_spots
    }

    fn sim(&mut self) -> Vec<((Loc, Minutes), (Loc, Minutes))> {
        let mut field_size = self.size;
        field_size.x -= 2;
        field_size.y -= 2;
        let step_cycle_number = (field_size.x as Minutes).lcm(&(field_size.y as Minutes));

        dbg!(step_cycle_number, field_size);

        let mut edges: Vec<((Loc, Minutes), (Loc, Minutes))> = Vec::with_capacity(1000);

        let mut empty_spots = self.get_empty_spots();

        for minutes in 0..step_cycle_number {
            self.blizzards_next();
            let current_empty_spots = self.get_empty_spots();

            let min = if minutes + 1 == step_cycle_number {
                0
            } else {
                minutes + 1
            };

            // if current_empty_spots.iter().any(|&b| b == end_node) {
            //     // println!("Insert endnode");
            //     edges.push(((end_node, min), end));
            // }

            for prev_spot in &empty_spots {
                for search_dir in 0..5 {
                    let mut search_spot = *prev_spot;

                    match search_dir {
                        0 => {
                            if search_spot.x == self.size.x - 1 {
                                continue;
                            }
                            search_spot.x += 1
                        }
                        1 => {
                            if search_spot.y == self.size.y - 1 {
                                continue;
                            };
                            search_spot.y += 1
                        }
                        2 => {
                            if search_spot.y == 0 {
                                continue;
                            };
                            search_spot.y -= 1
                        }
                        3 => {
                            if search_spot.x == 0 {
                                continue;
                            };
                            search_spot.x -= 1
                        }
                        _ => (),
                    }

                    if current_empty_spots.iter().any(|&b| b == search_spot) {
                        let spot = (search_spot, min);
                        let prev = (prev_spot.clone(), minutes);

                        edges.push((prev, spot));

                        // if spot.0 == end_node {
                        //     edges.push(((end_node, min), end));
                        // }
                    }
                }
            }
            empty_spots = current_empty_spots;
        }
        edges
    }

    fn draw(&self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let loc = Loc { x, y };
                let blis: Vec<Direction> = self
                    .blizzards
                    .iter()
                    .filter(|b| b.pos == loc)
                    .map(|b| b.dir)
                    .collect();

                let c = match blis.len() {
                    0 => {
                        if self.pos == loc {
                            'E'
                        } else {
                            '.'
                        }
                    }
                    1 => blis[0].as_char(),
                    e => char::from(b'0' + e as u8),
                };

                print!("{c}");
            }
            println!();
        }
    }
}

fn find_path(
    graph: &GraphMap<Node, u16, Directed>,
    start_point: Node,
    end_point: Loc,
) -> Option<(Node, u16)> {
    let start_time_begin = Instant::now();
    let ret = dijkstra(&graph, start_point, None, |_| 1_u16);
    let end_pathfind = start_time_begin.elapsed();

    let start_time = Instant::now();
    let ans = ret
        .iter()
        .filter(|(loc, _v)| loc.0 == end_point)
        .min_by_key(|(_, n)| *n);

    println!(
        "ans : {ans:?} in pathfind: {} mS, min: {} uS, total {} mS",
        end_pathfind.as_millis(),
        start_time.elapsed().as_micros(),
        start_time_begin.elapsed().as_millis(),
    );

    if let Some(ans) = ans {
        Some((*ans.0, *ans.1))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut total = 0;
        let start_begin = Instant::now();

        let mut sim = Sim::new(INPUT);

        let end = start_begin.elapsed();
        println!("Loaded map: {} uS", end.as_micros());

        let start = Instant::now();
        let edges = sim.sim();
        let end = start.elapsed();
        println!("generate edges: {} uS", end.as_micros());

        let start = Instant::now();
        let graph: GraphMap<Node, u16, Directed> = DiGraphMap::from_edges(&edges);
        let end = start.elapsed();
        println!("create graph: {} uS", end.as_micros());

        // let mut f = std::fs::File::create("graph.dot").unwrap();
        // f.write_all(format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel])).as_bytes())
        //     .unwrap();

        let end_point = Loc {
            x: sim.size.x - 2,
            y: sim.size.y - 1,
        };

        let start_point = (Loc { x: 1, y: 0 }, 0_u16);

        let ans = find_path(&graph, start_point, end_point);

        assert_eq!(ans, Some(((end_point, 6), 18)));

        total += ans.unwrap().1;

        // Go Back
        let start_point: Node = ans.unwrap().0;

        let end_point = Loc { x: 1, y: 0 };

        let ans = find_path(&graph, start_point, end_point);

        println!(
            "ans 2 : {ans:?} in {} mS",
            start_begin.elapsed().as_millis()
        );
        assert_eq!(ans, Some(((end_point, 5), 23)));

        total += ans.unwrap().1;

        // Go Back Again
        let start_point: Node = ans.unwrap().0;

        let end_point = Loc {
            x: sim.size.x - 2,
            y: sim.size.y - 1,
        };

        let ans = find_path(&graph, start_point, end_point);

        println!("ans 3 : {ans:?} in {} mS", end.as_micros());
        assert_eq!(ans, Some(((end_point, 6), 13)));

        total += ans.unwrap().1;

        println!("Total: {total} in {} mS", start_begin.elapsed().as_millis());

        assert_eq!(total, 54);
    }
}
