use std::io::{Read, Write};
use std::time::Instant;

type Item = u64;

#[derive(Debug, Clone)]
enum Operations {
    Add(Item),
    MulOld,
    Mul(Item),
}

#[derive(Debug, Clone)]
struct Monkey {
    Items: Vec<Item>,
    Operation: Operations,
    Test: Item,
    Throw: [usize; 2],
    Inspected: usize,
}

impl Monkey {
    fn new() -> Self {
        Self {
            Items: Vec::with_capacity(10),
            Operation: Operations::MulOld,
            Test: 0,
            Throw: [0, 0],
            Inspected: 0,
        }
    }
}

impl Default for Monkey {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct Day11(Vec<Monkey>);

impl Day11 {
    // Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    fn from(input: &str) -> Self {
        let mut monkeys = Vec::with_capacity(7);

        let mut monkey = Monkey::default();

        for line in input.split_terminator("\n") {
            let line = line.trim();
            //println!("{line}");
            if line.starts_with("Monkey ") {
                monkey = Monkey::default();
            }

            let mut words = line.split(" ");

            if line.starts_with("Starting items: ") {
                monkey.Items = words
                    .skip(2)
                    .map(|w| w.trim_end_matches(','))
                    .map(|w| w.parse::<Item>().unwrap())
                    .collect();
            } else if line.starts_with("Operation: new") {
                let sign = words.nth(4).unwrap().clone();
                let number = words.next().unwrap();
                monkey.Operation = match (sign, number) {
                    ("*", "old") => Operations::MulOld,
                    ("*", n) => Operations::Mul(n.parse().unwrap()),
                    ("+", n) => Operations::Add(n.parse().unwrap()),
                    (a, b) => panic!("No match {a}, {b}"),
                }
            } else if line.starts_with("Test: divisible by") {
                monkey.Test = words.skip(3).next().unwrap().parse::<Item>().unwrap();
            } else if line.starts_with("If true: throw to monkey") {
                monkey.Throw[0] = words.skip(5).next().unwrap().parse::<usize>().unwrap();
            } else if line.starts_with("If false: throw to monkey") {
                monkey.Throw[1] = words.skip(5).next().unwrap().parse::<usize>().unwrap();
                monkeys.push(monkey.clone());
            }
        }
        //println!("{monkeys:#?}");
        Self(monkeys)
    }

    fn next(&mut self) {
        for m in 0..self.0.len() {
            //println!("Monkey {m}:");
            let monkey = self.0.get(m).unwrap().clone();

            self.0[m].Items.clear();

            self.0[m].Inspected += monkey.Items.len();

            for item in monkey.Items {
                //println!("  Monkey inspects an item with a worry level of {item}.");
                let mut level = match monkey.Operation {
                    Operations::Add(n) => {
                        let ans = item + n;
                        //println!("   Worry level is increases by {n} to {ans}");
                        ans
                    }
                    Operations::MulOld => {
                        let ans = item * item;
                        //println!("   Worry level is multiplied by itself to {ans}");
                        ans
                    }
                    Operations::Mul(n) => {
                        let ans = item * n;
                        //println!("   Worry level is multiplied by {n} to {ans}");
                        ans
                    }
                };
                level /= 3;
                // println!(
                //     "    Monkey gets bored with item. Worry level is divided by 3 to {level}."
                // );

                let other = if level % monkey.Test == 0 {
                    //println!("    Current worry level is not divisible by {}.", monkey.Test);
                    monkey.Throw[0]
                } else {
                    //println!("    Current worry level is divisible by {}.", monkey.Test);
                    monkey.Throw[1]
                };
                //println!("    Item with worry level {level} is thrown to monkey {other}.");
                self.0[other].Items.push(level);
            }
        }
    }


    fn next_part2(&mut self, lcm: Item) {
        
        for m in 0..self.0.len() {
            //println!("Monkey {m}:");
            let monkey = self.0.get(m).unwrap().clone();

            self.0[m].Items.clear();

            self.0[m].Inspected += monkey.Items.len();

            for item in monkey.Items {
                //println!("  Monkey inspects an item with a worry level of {item}.");
                let level = match monkey.Operation {
                    Operations::Add(n) => {
                        let ans = item + n;
                        //println!("   Worry level is increases by {n} to {ans}");
                        ans
                    }
                    Operations::MulOld => {
                        let ans = item * item;
                        //println!("   Worry level is multiplied by itself to {ans}");
                        ans
                    }
                    Operations::Mul(n) => {
                        let ans = item * n;
                        //println!("   Worry level is multiplied by {n} to {ans}");
                        ans
                    }
                };
                // println!(
                //     "    Monkey gets bored with item. Worry level is divided by 3 to {level}."
                // );

                let other = if level % monkey.Test == 0 {
                    //println!("    Current worry level is not divisible by {}.", monkey.Test);
                    monkey.Throw[0]
                } else {
                    //println!("    Current worry level is divisible by {}.", monkey.Test);
                    monkey.Throw[1]
                };
                //println!("    Item with worry level {level} is thrown to monkey {other}.");
                self.0[other].Items.push(level % lcm);
            }
        }
    }

    fn end(&self) -> usize {
        let mut hs: Vec<usize> = self.0.iter().map(|m| m.Inspected).collect();

        for (idx, times) in hs.iter().enumerate() {
            println!("Monkey {idx} inspected items {} times.", times);
        }
        
        hs.sort();

        hs[hs.len()-1] * hs[hs.len()-2]
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
    println!("Part2: {} in {} uS", total, end.as_micros());

    println!("Total in {} uS", start_begin.elapsed().as_micros());
}

fn part1(input: &str) -> usize {
    let mut monkeys = Day11::from(input);

    for _ in 0..20 {
        monkeys.next();
    }

    monkeys.end()
}

fn part2(input: &str) -> usize {
    let mut monkeys = Day11::from(input);

    let lcm = monkeys.0.iter().map(|n| n.Test).fold(1, |acc, v| acc * v);
    println!("lcm {lcm}");

    for n in 0..10000 {
        //println!("round: {n}");
        monkeys.next_part2(lcm);
    }

    monkeys.end()
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3
  
  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0
  
  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3
  
  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1";

    #[test]
    fn score() {
        // let score = part1(INPUT);

        // println!("Score: {score}");

        // assert_eq!(score, 1651);
        let mut monkeys = Day11::from(INPUT);
        println!("{monkeys:#?}");

        for _ in 0..20 {
            monkeys.next();
        }

        assert_eq!(monkeys.end(), 10605);
    }

    //#[ignore = "reason"]
    #[test]
    fn score2() {
        let mut monkeys = Day11::from(INPUT);


        let lcm = monkeys.0.iter().map(|n| n.Test).fold(1, |acc, v| acc * v);
        println!("lcm {lcm}");
    
        for n in 0..10000 {
            //println!("round: {n}");
            monkeys.next_part2(lcm);
        }
    
        assert_eq!(monkeys.end(), 2713310158);
    }
}
