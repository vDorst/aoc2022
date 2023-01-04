use petgraph::algo::floyd_warshall;
use petgraph::visit::{GraphRef, IntoNeighbors, VisitMap, Visitable};
use petgraph::{
    prelude::*,
    visit::{GraphProp, IntoEdgeReferences, NodeIndexable},
    Directed, Graph,
};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;
use std::{fmt::Display, num::NonZeroU8};

use petgraph::{
    algo::dijkstra,
    dot::{Config, Dot},
};
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Idx(u8);

type Score = u32;

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
        Idx(u8::try_from(idx).expect("Index higher u8::max {idx}"))
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

type NodeG = (String, Flow);

type Flow = u16;

type G = Graph<NodeG, Flow, Undirected>;

#[derive(Debug)]
struct Day16(G);

impl Day16 {
    // Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    fn from(input: &str) -> Self {
        let mut graph = G::new_undirected();

        for line in input.split_terminator("\n") {
            let mut words = line.split(" ");

            let node_name = words.nth(1).unwrap().to_string();

            let rate = words.nth(2).unwrap()[5..]
                .trim_end_matches(';')
                .parse::<Flow>()
                .unwrap();

            let valves: Vec<String> = words
                .skip(4)
                .map(|w| w.trim_end_matches(',').to_string())
                .collect();

            //println!("{node_name:?}, {rate}, {valves:?}");

            let from_idx =
                if let Some(val) = graph.node_indices().find(|n| graph[*n].0 == node_name) {
                    graph[val].1 = rate;
                    val
                } else {
                    graph.add_node((node_name, rate))
                };

            for node_to in valves {
                let to_idx =
                    if let Some(val) = graph.node_indices().find(|n| graph[*n].0 == node_to) {
                        val
                    } else {
                        graph.add_node((node_to, 0))
                    };

                graph.add_edge(from_idx, to_idx, 1);
            }
        }
        Self(graph)
    }
}

#[derive(Debug, Clone)]
struct Search {
    node: NodeIndex,
    distance: HashMap<NodeIndex, Score>,
    flow: u8,
}

pub fn bfs_30_min<G, N, VM, B>(graph: G, start: N) -> Option<(N, Score)>
where
    N: Copy + PartialEq,
    VM: VisitMap<N>,
    G: GraphRef + Visitable<NodeId = N, Map = VM> + NodeIndexable + IntoNeighbors<NodeId = N>,
{
    let mut vm = graph.visit_map();
    vm.visit(start);

    let mut dist = vec![Score::MAX; graph.node_bound()];
    dist[graph.to_index(start)] = 0;

    let mut queue: VecDeque<G::NodeId> = VecDeque::new();
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        for v in graph.neighbors(current) {
            if vm.visit(v) {
                let node: N = v;
                let dis = dist[graph.to_index(current)] + 1;
                queue.push_back(v);
                dist[graph.to_index(v)] = dis;
            }
        }
    }

    None
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

trait Scoring {
    fn score(&self, dis: Score) -> Score;
}

impl Scoring for Score {
    fn score(&self, dis: Score) -> Score {
        if *self > dis {
            return *self - dis - 1;
        }
        0
    }
}

fn main() {
    let start_begin = Instant::now();

    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = String::with_capacity(4096);
    f.read_to_string(&mut input).unwrap();

    let start = Instant::now();
    let total = part1(&input);
    let end = start.elapsed();
    println!("Part1: {} in {} uS", total, end.as_micros());

    let start = Instant::now();
    let total = part2(&input);
    let end = start.elapsed();
    println!("Part2: {:?} = {} in {} uS", total, total.iter().sum::<u32>(), end.as_micros());

    println!("Total in {} uS", start_begin.elapsed().as_micros());
}

fn rec(data: &Vec<Search>, values_open: u16, prev: NodeIndex, minutes: u32) -> (Score, u16) {
    let prev_dis = data.get(prev.index()).unwrap();

    let mut score: Score = 0;
    for (bit_idx, node) in data.iter().enumerate() {
        let bit = 1 << bit_idx;
        if values_open & bit != 0 {
            continue;
        }
        let distance = *prev_dis.distance.get(&node.node).unwrap();
        let Some(rem_minutes) = minutes.checked_sub(distance + 1) else { continue;} ;
        score = score.max(
            rec(
                data,
                values_open | bit,
                                NodeIndex::from(bit_idx as u32),
                rem_minutes,
            ).0 + (node.flow as u32 * rem_minutes),
        );
    }

    (score, values_open)
}

fn rec2(data: &Vec<Search>, values_open: u16, prev: [NodeIndex;2], minutes: [u32;2]) -> (Score, u16) {
    let player = if minutes[0] >= minutes[1] {
        0
    } else {
        1
    };
    let prev_dis = data.get(prev[player].index()).unwrap();

    let mut score: Score = 0;
    for (bit_idx, node) in data.iter().enumerate() {
        let bit = 1 << bit_idx;
        if values_open & bit != 0 {
            continue;
        }
        let distance = *prev_dis.distance.get(&node.node).unwrap();
        let Some(rem_minutes) = minutes[player].checked_sub(distance + 1) else { continue;};
        let mut ni = prev;
        ni[player] = NodeIndex::from(bit_idx as u32);
        let mut rm = minutes;
        rm[player] = rem_minutes;
        score = score.max(
            rec2(
                data,
                values_open | bit,
                ni,
                rm,
            ).0 + (node.flow as u32 * rem_minutes),
        );
    }

    (score, values_open)
}


fn part1(input: &str) -> Score {
    let graph = Day16::from(input);
    // let mut f = std::fs::File::create("graph.dot").unwrap();
    // f.write_all(format!("{:?}", Dot::with_config(&graph.0, &[])).as_bytes())
    //     .unwrap();

    let start_idx = NodeIndex::from(graph.0.node_weights().position(|n| n.0 == "AA").unwrap() as u32);

    let s: Vec<Search> = graph
        .0
        .node_indices()
        .filter(|n| *n == start_idx || graph.0[*n].1 > 0)
        .map(|n| Search {
            node: n,
            distance: dijkstra(&graph.0, n, None, |_| 1),
            flow: graph.0[n].1 as u8,
        })
        .collect();

    println!("combi {}, {}", s.len(), factorial(s.len() - 1));

    rec(&s, 0, NodeIndex::from(s.iter().position(|n| n.node == start_idx).unwrap() as u32), 30).0
}

fn part2(input: &str) -> Vec<Score> {
    let graph = Day16::from(input);
    // let mut f = std::fs::File::create("graph.dot").unwrap();
    // f.write_all(format!("{:?}", Dot::with_config(&graph.0, &[])).as_bytes())
    //     .unwrap();

    let start_idx = NodeIndex::from(graph.0.node_weights().position(|n| n.0 == "AA").unwrap() as u32);

    let s: Vec<Search> = graph
        .0
        .node_indices()
        .filter(|n| *n == start_idx || graph.0[*n].1 > 0)
        .map(|n| Search {
            node: n,
            distance: dijkstra(&graph.0, n, None, |_| 1),
            flow: graph.0[n].1 as u8,
        })
        .collect();

    println!("combi {}, {}", s.len(), factorial(s.len() - 1));

    let mut total = Vec::<Score>::with_capacity(2);

    let si = NodeIndex::from(s.iter().position(|n| n.node == start_idx).unwrap() as u32);

    let (score, valve) = rec2(&s, 0, [si;2 ], [26; 2]);
    total.push(score);

    
    total
}

fn factorial(n: usize) -> usize {
    let mut ret = 1;
    for n in 1..n {
        ret *= n;
    }
    ret
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

const INPUT_B: &str = "Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";


    #[test]
    fn score() {
        let score = part1(INPUT);

        println!("Score: {score}");

        assert_eq!(score, 1651);

        let score = part1(INPUT_B);

        println!("Score: {score}");

        assert_eq!(score, 1651);
    }

    #[test]
    fn score2() {
        let score = part2(INPUT);

        println!("Score: {score:?} {}", score.iter().sum::<Score>());

        assert_eq!(score[0], 1707);
    }
}
