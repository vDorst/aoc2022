use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use std::{fs::File, io::Read, time::Instant};

type Item = u8;

#[derive(Debug, Eq)]
enum Packet {
    List(Vec<Packet>),
    Number(Item),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Number(n1), Self::List(l2)) => &vec![Packet::Number(*n1)] == l2,
            (Self::List(l1), Self::Number(n2)) => l1 == &vec![Packet::Number(*n2)],
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Number(n1), Packet::Number(n2)) => n1.cmp(&n2),
            (Packet::Number(n1), Packet::List(l2)) => vec![Packet::Number(*n1)].cmp(l2),
            (Packet::List(l1), Packet::Number(n2)) => l1.cmp(&vec![Packet::Number(*n2)]),
            (Packet::List(l1), Packet::List(l2)) => l1.cmp(l2),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Pair {
    l: Packet,
    r: Packet,
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), packet), tag("]"))
            .map(|list| Packet::List(list)),
        complete::u8.map(|n| Packet::Number(n)),
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(packet, newline, packet).map(|(l, r)| Pair { l, r }),
    )(input)
}

fn part1(list: &Vec<Pair>) -> usize {
    list.iter()
        .zip(1..)
        .filter_map(|(Pair { l, r }, i)| match l.cmp(&r) {
            std::cmp::Ordering::Less => Some(i),
            std::cmp::Ordering::Equal => todo!(),
            std::cmp::Ordering::Greater => None,
        })
        .sum()
}

fn part2(list: &Vec<Pair>) -> usize {
    let mut nl: Vec<&Packet> = list.iter().flat_map(|Pair { l, r }| [l, r]).collect();
    nl.sort();

    nl.iter()
        .zip(1..)
        //.inspect(|p| println!("{p:?}"))
        .filter_map(|(p, i)| {
            if **p == Packet::List(vec![Packet::List(vec![Packet::Number(2)])])
                || **p == Packet::List(vec![Packet::List(vec![Packet::Number(6)])])
            {
                Some(i)
            } else {
                None
            }
        })
        .product()
}

fn main() {
    let start_begin = Instant::now();

    let mut f = File::open("input/input.txt").unwrap();
    let mut input = String::with_capacity(4096);
    f.read_to_string(&mut input).unwrap();

    let (_, mut list) = parse(&input).unwrap();

    let start = Instant::now();
    let total = part1(&list);
    let end = start.elapsed();
    println!("Part1: {} in {} uS", total, end.as_micros());

    let start = Instant::now();

    list.push(Pair {
        l: Packet::List(vec![Packet::List(vec![Packet::Number(2)])]),
        r: Packet::List(vec![Packet::List(vec![Packet::Number(6)])]),
    });

    let total = part2(&list);
    let end = start.elapsed();
    println!("Part2: {} in {} uS", total, end.as_micros());

    println!("Total in {} uS", start_begin.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn example_part1() {
        let (_, list) = parse(INPUT).unwrap();
        assert_eq!(part1(&list), 13);
    }

    #[test]
    fn example_part2() {
        let (_, mut list) = parse(INPUT).unwrap();

        list.push(Pair {
            l: Packet::List(vec![Packet::List(vec![Packet::Number(2)])]),
            r: Packet::List(vec![Packet::List(vec![Packet::Number(6)])]),
        });

        assert_eq!(part2(&list), 140);
    }
}
