use std::{fmt::Display, io::Read, num::NonZeroU8};

type Score = u32;
type IdxType = u16;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

    fn from(idx: usize) -> Self {
        Idx(IdxType::try_from(idx).expect("Index higher u8::max {idx}"))
    }

    fn next(&mut self) {
        self.0 += 1;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Dis(NonZeroU8);

#[derive(Debug, PartialEq, Eq)]
struct Node {
    name: String,
    vertex: Vec<(Idx, Dis)>,
    flow: u8,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            vertex: Vec::with_capacity(3),
            flow: 0,
        }
    }
}

type Dist = u16;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    value: u8,
    distance: Dist,
    processed: bool,
    prev: Idx,
}

impl Point {
    fn can_go(&self, rhs: &Point) -> bool {
        if rhs.processed {
            return false;
        }
        let ret = self.valid_negbor(rhs);
        //print!("- {} vs {}: {ret} -", char::from(self.value), char::from(rhs.value) );
        ret
    }

    fn valid_negbor(&self, rhs: &Point) -> bool {
        // next value +1 higher, equel or lower
        let s = if self.value == b'E' {
            b'z' + 1
        } else {
            self.value
        };
        let e = if rhs.value == b'E' {
            b'z' + 1
        } else {
            rhs.value
        };
        if e.abs_diff(s) <= 1 { 
            return true;
        }
        if s >= e {
           return true;
        }
        false
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Nodes {
    points: Vec<Point>,
    size: (usize, usize),
    current: Idx,
    start: Idx,
    end: Idx,
}

impl Nodes {
    fn from(input: &[u8]) -> Self {
        let mut points = Vec::with_capacity(30);

        let mut y = 0;
        for line in input.split(|v| *v == b'\n') {
            for p in line {
                points.push(Point {
                    value: *p,
                    distance: Dist::MAX,
                    processed: false,
                    prev: Idx(0),
                })
            }
            y += 1;
        }

        let (idx, current_point) = points
            .iter_mut()
            .enumerate()
            .find(|(_idx, v)| v.value == b'S')
            .unwrap();

        current_point.distance = 0;
        current_point.value = b'a';

        let (end_idx, end_point) = points
        .iter_mut()
        .enumerate()
        .find(|(_idx, v)| v.value == b'E')
        .unwrap();

        end_point.value = b'z';

        Self {
            size: (points.len() / y, y),
            points,
            current: Idx::from(idx),
            start: Idx::from(idx),
            end: Idx::from(end_idx),
        }
    }

    fn get(&self, idx: Idx) -> &Point {
        &self.points[idx.as_usize()]
    }

    fn get_mut(&mut self, idx: Idx) -> &mut Point {
        &mut self.points[idx.as_usize()]
    }

    fn pos(&self, idx: Idx) -> (usize, usize) {
        let point_idx = idx.as_usize();
        let max_x = self.size.0;

        let y = point_idx / max_x;
        let x = point_idx % max_x;
        (x, y)
    }

    fn get_neighbors(&self, idx: Option<Idx>) -> Vec<Idx> {
        let point_idx = if let Some(idx) = idx { idx.as_usize() } else {self.current.as_usize() };
        let max_x = self.size.0;

        let y = point_idx / max_x;
        let x = point_idx % max_x;

        let mut neighbors = Vec::<Idx>::with_capacity(4);

        if x != 0 {
            neighbors.push(Idx::from(point_idx - 1))
        }
        if x != max_x - 1 {
            neighbors.push(Idx::from(point_idx + 1))
        }

        if y != 0 {
            neighbors.push(Idx::from(point_idx - max_x))
        }
        if y != self.size.1 - 1 {
            neighbors.push(Idx::from(point_idx + max_x))
        }

        neighbors
    }

    fn distances(&mut self) -> Idx {
        'search: loop {
            let mut min: Option<(Dist, Idx)> = None;


            if self.get_neighbors(Some(self.end)).iter().all(|n| 
                self.get(*n).processed ) {
                break 'search;
            }
            // let curr_point = self.get(self.current);

            // Find unvised minium score value;
            for (point_idx, point) in self.points.iter().enumerate() {
                if !point.processed {
                    if let Some(min) = &mut min {
                        if point.distance < min.0 {
                            *min = (point.distance, Idx::from(point_idx));
                        }
                    } else {
                        min = Some((point.distance, Idx::from(point_idx)));
                    }
                }
            }
            if let Some(min) = min {
                self.current = min.1;
                // print!("Dis {} -- ", min.0);
                //self.print_current();
            } else {
                break 'search;
            }

            let idx = self.current;

            self.get_mut(idx).processed = true;

            let curr = self.get(idx).clone();

            if curr.distance == Dist::MAX {
                //continue;
                break 'search;
            }

            let distance_to_next = curr.distance + 1;

            for link in self.get_neighbors(None) {
                let next = self.get_mut(link);

                if curr.can_go(next) && distance_to_next < next.distance {
                    // print!("Update: {link}: {next:?}");
                    next.distance = distance_to_next;
                    next.prev = idx;
                    // println!("-> {next:?}");
                }
            }
        }

        // for link in &path {
        //     println!("{link:?}");
        // }

        self.end
    }

    fn print_current(&self) {
        let point_idx = self.current.as_usize();
        let max_x = self.size.0;

        let y = point_idx / max_x;
        let x = point_idx % max_x;

        println!("{point_idx} {x}x{y}");
    }

    fn print_map(&self) {
        let mut idx = Idx(0);

        for points in self.points.chunks_exact(self.size.0) {
            for point in points {
                let color = if point.processed {
                    "\x1b[38;5;21m"
                } else if point.distance == Dist::MAX {
                    "\x1b[38;5;1m"
                } else {
                    "\x1b[38;5;40m"
                };
                let c = if idx == self.end {
                     b'E'
                } else if idx == self.start {
                    b'S' } else { point.value };
                print!("{color}{}", char::from(c));
                idx.next();
            }
            println!("");
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Walk {
    /// Previous node idx to get to this point
    prev: Idx,
    /// Distance from the start node
    distance: u8,
    /// Is this node already processed.
    processed: bool,
}

const INPUT: &[u8] = b"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

//   01234567
// 0 Sabqponm
// 1 abcryxxl
// 2 accszExk
// 3 acctuvwj
// 4 abdefghi

fn main() {
    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = Vec::<u8>::with_capacity(1_000_000);
    f.read_to_end(&mut input).unwrap();

    println!("len: {}", input.len());

    let mut nodes = Nodes::from(&input);

    let end = nodes.distances();
    let end_point = nodes.get(end);
    println!("steps: {}", end_point.distance);

    nodes.print_map();

    let mut best = Dist::MAX;
    for pos in (0..nodes.points.len()).step_by(nodes.size.0) {

        let mut nodes = Nodes::from(&input);
        
        nodes.get_mut(nodes.current).distance = Dist::MAX;
        nodes.current = Idx::from(pos);

        nodes.get_mut(nodes.current).distance = 0;

        let end = nodes.distances();

        let end_point = nodes.get(end);
        println!("steps: {}", end_point.distance);

        if end_point.distance < best { best = end_point.distance };
    }

    println!("Best: {}", best);

}

#[cfg(test)]

mod tests {

    use super::{Idx, Nodes, INPUT};

    #[test]
    fn test_distance() {
        let mut nodes = Nodes::from(INPUT);
        assert_eq!(nodes.size, (8, 5));

        let end = nodes.distances();

        assert_eq!(end, Idx::from(21));

        let mut p = end;

        loop {
            let node = nodes.get(p);
            //let (x, y) = nodes.pos(p);

            // print!("{x}x{y} -> ");

            if p == Idx(0) {
                break;
            }

            p = node.prev;
        }

        println!("");


        let end_point = nodes.get(end);

        nodes.print_map();

        assert_eq!(end_point.distance, 31);

    }
}
