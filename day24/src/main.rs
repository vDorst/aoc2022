use ahash::AHashSet;
use itertools::Itertools;
use num::Integer;
use petgraph::{
    prelude::{DiGraphMap, GraphMap},
    visit::{GraphRef, IntoNeighbors, NodeIndexable, VisitMap, Visitable},
    Directed,
};
use std::{collections::VecDeque, io::Read, time::Instant};

type Minutes = u16;

#[derive(Debug, Clone, Copy, Ord, PartialEq, PartialOrd, Eq, Hash)]
struct Node(Loc, Minutes);

impl Location for Node {
    fn x(&self) -> u8 {
        self.0.x()
    }

    fn y(&self) -> u8 {
        self.0.y()
    }
}

fn part1(graph: &GraphMap<Node, Minutes, Directed>, sim: &Sim) -> Minutes {
    let start_begin = Instant::now();

    let end_point = Loc {
        x: sim.size.x - 2,
        y: sim.size.y - 1,
    };

    let start_point = Node(Loc { x: 1, y: 0 }, 0_u16);

    let ans = find_path(graph, start_point, end_point);

    println!("Total: {ans:?} in {} mS", start_begin.elapsed().as_millis());

    ans.unwrap().1
}

fn part2(graph: &GraphMap<Node, Minutes, Directed>, map_data: &Sim) -> Vec<Minutes> {
    let mut total = Vec::<Minutes>::with_capacity(3);
    let start_begin = Instant::now();

    let end_point = Loc {
        x: map_data.size.x - 2,
        y: map_data.size.y - 1,
    };

    let start_point = Node(Loc { x: 1, y: 0 }, 0_u16);

    let ans = find_path(graph, start_point, end_point);

    total.push(ans.unwrap().1);

    // Go Back
    let start_point: Node = ans.unwrap().0;

    let end_point = Loc { x: 1, y: 0 };

    let ans = find_path(graph, start_point, end_point);

    total.push(ans.unwrap().1);

    // Go Back Again
    let start_point: Node = ans.unwrap().0;

    let end_point = Loc {
        x: map_data.size.x - 2,
        y: map_data.size.y - 1,
    };

    let ans = find_path(graph, start_point, end_point);

    total.push(ans.unwrap().1);

    let sum: u16 = total.iter().sum();
    println!(
        "Total: {total:?} = {sum} in {} mS",
        start_begin.elapsed().as_millis()
    );

    total
}

fn main() {
    let start_begin = Instant::now();

    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = Vec::with_capacity(1_000_000);
    f.read_to_end(&mut input).unwrap();

    let mut map_data = Sim::new(&input);

    let end = start_begin.elapsed();
    println!("Loaded map: {} uS", end.as_micros());

    let start = Instant::now();
    let graph = map_data.sim();
    let end = start.elapsed();
    println!("generate edges: {} uS", end.as_micros());

    // let start = Instant::now();
    // let graph: GraphMap<Node, Minutes, Directed> = DiGraphMap::from_edges(&edges);
    // let end = start.elapsed();
    // println!("create graph: {} uS", end.as_micros());

    let answer = part1(&graph, &map_data);
    println!("Part1: {answer}");

    let answer = part2(&graph, &map_data);
    let sum: u16 = answer.iter().sum();
    println!("Part2: sum: {answer:?} = {sum}");
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd, Hash)]
struct Loc {
    x: u8,
    y: u8,
}

impl Location for Loc {
    fn x(&self) -> u8 {
        self.x
    }

    fn y(&self) -> u8 {
        self.y
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Wall,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Blizzard {
    pos: Loc,
    dir: Direction,
}

#[derive(Debug)]
struct Sim {
    blizzards: Vec<Blizzard>,
    size: Loc,
}

impl Sim {
    fn new(input: &[u8]) -> Self {
        let mut sim = Sim {
            blizzards: Vec::with_capacity(input.len()),
            size: Loc { x: 0, y: 0 },
        };

        let mut size_y = 0;
        let mut size_x = 0;
        for (y, line) in (0_u8..).zip(input.split(|&v| v == b'\n')) {
            size_x = line.len();
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
            size_y += 1;
        }

        sim.size = Loc {
            x: u8::try_from(size_x).unwrap(),
            y: size_y,
        };

        println!(
            "blizzards len {} size = {}",
            sim.blizzards.len(),
            sim.blizzards.len() * std::mem::size_of::<Blizzard>()
        );

        sim
    }

    fn blizzards_next(&mut self) {
        for blis in &mut self.blizzards {
            match blis.dir {
                Direction::Up => {
                    blis.pos.y -= 1;
                    if blis.pos.y == 0 {
                        blis.pos.y = self.size.y - 2;
                    }
                }
                Direction::Down => {
                    blis.pos.y += 1;
                    if blis.pos.y == self.size.y - 1 {
                        blis.pos.y = 1;
                    }
                }
                Direction::Left => {
                    blis.pos.x -= 1;
                    if blis.pos.x == 0 {
                        blis.pos.x = self.size.x - 2;
                    }
                }
                Direction::Right => {
                    blis.pos.x += 1;
                    if blis.pos.x == self.size.x - 1 {
                        blis.pos.x = 1;
                    }
                }
                Direction::Wall => (),
            }
        }
    }

    fn get_empty_spots(&self) -> AHashSet<Loc> {
        let mut empty_spots: AHashSet<Loc> = (0..self.size.y)
            .cartesian_product(1..(self.size.x - 1))
            .map(|(y, x)| Loc { x, y })
            .collect();

        for blis in &self.blizzards {
            empty_spots.remove(&blis.pos);
        }
        // dbg!(empty_spots.len());
        empty_spots
    }

    fn sim(&mut self) -> GraphMap<Node, u16, Directed> {
        let mut field_size = self.size;
        field_size.x -= 2;
        field_size.y -= 2;
        let step_cycle_number = (Minutes::try_from(field_size.x).unwrap()).lcm(&(Minutes::try_from(field_size.y).unwrap()));

        // dbg!(step_cycle_number, field_size);

        let mut graph: GraphMap<Node, u16, Directed> =
            DiGraphMap::with_capacity(900_000, 2_000_000);

        let mut empty_spots = self.get_empty_spots();

        for minutes in 0..step_cycle_number {
            self.blizzards_next();
            let current_empty_spots = self.get_empty_spots();

            let min = (minutes + 1) % step_cycle_number;

            for prev_spot in &empty_spots {
                for search_dir in [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::Wall,
                ] {
                    let mut search_spot = *prev_spot;

                    match search_dir {
                        Direction::Right => {
                            if search_spot.x >= self.size.x - 2 {
                                continue;
                            }
                            search_spot.x += 1;
                        }
                        Direction::Down => {
                            if search_spot.y == self.size.y - 1 {
                                continue;
                            };
                            search_spot.y += 1;
                        }
                        Direction::Up => {
                            if search_spot.y == 0 {
                                continue;
                            };
                            search_spot.y -= 1;
                        }
                        Direction::Left => {
                            if search_spot.x <= 1 {
                                continue;
                            };
                            search_spot.x -= 1;
                        }
                        Direction::Wall => (),
                    }

                    if current_empty_spots.contains(&search_spot) {
                        let spot = Node(search_spot, min);
                        let prev = Node(*prev_spot, minutes);

                        graph.add_edge(prev, spot, 1);
                    }
                }
            }
            empty_spots = current_empty_spots;
        }
        println!(
            "Graph: Nodes {} Egdes {}",
            graph.node_count(),
            graph.edge_count()
        );

        // let mut remove_node = Vec::<Node>::with_capacity(10000);

        // loop {
        //     remove_node.clear();

        //     for node in graph.nodes() {
        //         if graph.neighbors_directed(node, petgraph::Direction::Incoming).next().is_none() {
        //             remove_node.push(node);
        //         }
        //         if graph.neighbors_directed(node, petgraph::Direction::Outgoing).next().is_none() {
        //             remove_node.push(node);
        //         }
        //     }
        //     if remove_node.is_empty() { break }

        //     for &node in &remove_node {
        //         graph.remove_node(node);
        //     }
        // }

        // println!("Graph: Nodes {} Egdes {}", graph.node_count(), graph.edge_count());
        // graph.nodes()

        graph
    }
}

fn find_path(
    graph: &GraphMap<Node, u16, Directed>,
    start_point: Node,
    end_point: Loc,
) -> Option<(Node, u16)> {
    let start_time_begin = Instant::now();

    let ans = bfs_with_goal(graph, start_point, &end_point);

    println!(
        "ans : {ans:?} in pathfind: {} uS",
        start_time_begin.elapsed().as_micros(),
    );

    ans
}

pub trait Location {
    fn x(&self) -> u8;
    fn y(&self) -> u8;
}

pub fn bfs_with_goal<G, N, VM, B>(graph: G, start: N, end: &B) -> Option<(N, Minutes)>
where
    N: Copy + PartialEq + Location,
    B: Location,
    VM: VisitMap<N>,
    G: GraphRef + Visitable<NodeId = N, Map = VM> + NodeIndexable + IntoNeighbors<NodeId = N>,
{
    let mut vm = graph.visit_map();
    vm.visit(start);

    let mut dist = vec![Minutes::MAX; graph.node_bound()];
    dist[graph.to_index(start)] = 0;

    let mut queue: VecDeque<G::NodeId> = VecDeque::new();
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        for v in graph.neighbors(current) {
            if vm.visit(v) {
                let node: N = v;
                let dis = dist[graph.to_index(current)] + 1;
                if node.x() == end.x() && node.y() == end.y() {
                    return Some((node, dis));
                }
                queue.push_back(v);
                dist[graph.to_index(v)] = dis;
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use petgraph::dot::{Config, Dot};
    use std::io::Write;

    use super::*;

    const INPUT: &[u8] = b"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_example1() {
        let mut total = 0;
        let start_begin = Instant::now();

        let mut sim = Sim::new(INPUT);

        let end = start_begin.elapsed();
        println!("Loaded map: {} uS", end.as_micros());

        let start = Instant::now();
        let graph = sim.sim();
        println!("generate edges: {} uS", start.elapsed().as_micros());

        // let start = Instant::now();
        // let graph: GraphMap<Node, u16, Directed> = DiGraphMap::from_edges(&edges);
        // println!("create graph: {} uS", start.elapsed().as_micros());

        let mut f = std::fs::File::create("graph.dot").unwrap();
        f.write_all(format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel])).as_bytes())
            .unwrap();

        let end_point = Loc {
            x: sim.size.x - 2,
            y: sim.size.y - 1,
        };

        let start_point = Node(Loc { x: 1, y: 0 }, 0_u16);

        let ans = find_path(&graph, start_point, end_point);

        assert_eq!(ans, Some((Node(end_point, 6), 18)));

        total += ans.unwrap().1;

        let start = Instant::now();
        let ans_bfs = bfs_with_goal(&graph, start_point, &end_point);
        println!(
            "Bfs search: {ans_bfs:?} in {} uS",
            start.elapsed().as_micros()
        );

        assert_eq!(ans_bfs, Some((Node(end_point, 6), 18)));

        // Go Back
        let start_point = ans.unwrap().0;

        let end_point = Loc { x: 1, y: 0 };

        let ans = find_path(&graph, start_point, end_point);

        println!(
            "ans 2 : {ans:?} in {} mS",
            start_begin.elapsed().as_millis()
        );
        assert_eq!(ans, Some((Node(end_point, 5), 23)));

        total += ans.unwrap().1;

        // Go Back Again
        let start_point: Node = ans.unwrap().0;

        let end_point = Loc {
            x: sim.size.x - 2,
            y: sim.size.y - 1,
        };

        let ans = find_path(&graph, start_point, end_point);

        println!("ans 3 : {ans:?} in {} mS", end.as_micros());
        assert_eq!(ans, Some((Node(end_point, 6), 13)));

        total += ans.unwrap().1;

        println!("Total: {total} in {} mS", start_begin.elapsed().as_millis());

        assert_eq!(total, 54);
    }
}
