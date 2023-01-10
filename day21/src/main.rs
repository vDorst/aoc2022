use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{self, line_ending, one_of},
        streaming::alpha1,
    },
    combinator::{eof, iterator},
    sequence::{delimited, terminated, tuple},
    IResult, Parser,
};
use std::{collections::HashMap, fs::File, io::Read, time::Instant};

#[derive(Debug, Clone)]
enum Ans<'a> {
    Var(&'a str),
    Num(i64),
}

#[derive(Debug)]
enum Input<'a> {
    Eq((&'a str, Ans<'a>, char, Ans<'a>)),
    Var((&'a str, i64)),
}

fn line(input: &str) -> IResult<&str, Input<'_>> {
    let (input, var) = alpha1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, out) = alt((
        complete::i64.map(|n| Input::Var((var, n))),
        tuple((
            alpha1,
            delimited(tag(" "), one_of("*+-/"), tag(" ")),
            alpha1,
        ))
        .map(|(left, op, right)| Input::Eq((var, Ans::Var(left), op, Ans::Var(right)))),
    ))(input)?;
    //println!("Line: {out:?}");
    Ok((input, out))
}

fn parse(input: &str) -> Vec<Input> {
    let ins = iterator(input, terminated(line, alt((line_ending, eof)))).collect::<Vec<_>>();

    ins
}

fn part1(input: &str) -> i64 {
    let mut list = parse(input);

    // let total = list.len();

    let mut numbers: HashMap<&str, i64> = list
        .iter()
        .filter_map(|e| {
            if let Input::Var((var, n)) = e {
                Some((*var, *n))
            } else {
                None
            }
        })
        .collect();
    list.retain(|e| if let Input::Eq(_n) = e { true } else { false });

    let mut index = 0;
    let ans: i64 = loop {
        if list.is_empty() {
            break 0;
        }
        let (var, left, op, right) = if let Some(Input::Eq(n)) = list.get_mut(index) {
            n
        } else {
            // println!("Can´t get {index}");
            index = 0;
            continue;
        };

        if let Ans::Var(v) = left {
            if let Some(n) = numbers.get(v) {
                *left = Ans::Num(*n);
            }
        }

        if let Ans::Var(v) = right {
            if let Some(n) = numbers.get(v) {
                *right = Ans::Num(*n);
            }
        }

        if let (Ans::Num(l), Ans::Num(r)) = (left, right) {
            let ans = match op {
                '*' => *l * *r,
                '+' => *l + *r,
                '/' => *l / *r,
                '-' => *l - *r,
                _ => panic!("No op!"),
            };
            if *var == "root" {
                break ans;
            }
            // println!("status: [{index}] {var} = {ans} -- {}/{total}", numbers.len() + 1);

            numbers.insert(var, ans);
            list.swap_remove(index);
        }

        index += 1;
    };

    ans
}

fn part2(input: &str) -> i64 {
    let mut list = parse(input);

    let total = list.len();

    let mut numbers: HashMap<&str, i64> = list
        .iter()
        .filter_map(|e| {
            if let Input::Var((var, n)) = e {
                Some((*var, *n))
            } else {
                None
            }
        })
        .collect();
    list.retain(|e| if let Input::Eq(_n) = e { true } else { false });

    numbers.remove("humn");

    let mut index = 0;
    let ans: (Ans, Ans) = loop {
        let (var, left, op, right) = if let Some(Input::Eq(n)) = list.get_mut(index) {
            n
        } else {
            // println!("Can´t get {index}");
            index = 0;
            continue;
        };

        if let Ans::Var(v) = left {
            if let Some(n) = numbers.get(v) {
                *left = Ans::Num(*n);
                if *var == "root" {
                    break (left.clone(), right.clone());
                }
            }
        }

        if let Ans::Var(v) = right {
            if let Some(n) = numbers.get(v) {
                *right = Ans::Num(*n);
                if *var == "root" {
                    break (left.clone(), right.clone());
                }
            }
        }

        if let (Ans::Num(l), Ans::Num(r)) = (left, right) {
            let ans = match op {
                '*' => *l * *r,
                '+' => *l + *r,
                '/' => *l / *r,
                '-' => *l - *r,
                _ => panic!("No op!"),
            };

            // println!("status: [{index}] {var} = {ans} -- {}/{total}", numbers.len() + 1);

            numbers.insert(var, ans);
            list.swap_remove(index);
        }

        index += 1;
    };

    // println!("Ans: {ans:?}");

    let name = if let Ans::Var(name) = ans.0 {
        name
    } else {
        panic!()
    };
    let num = if let Ans::Num(num) = ans.1 {
        num
    } else {
        panic!()
    };

    numbers.insert(name, num);
    list.swap_remove(index);

    let mut index = 0;
    let ans: i64 = loop {
        let (var, left, op, right) = if let Some(Input::Eq(n)) = list.get_mut(index) {
            n
        } else {
            // println!("Can´t get {index}");
            index = 0;
            continue;
        };

        if let Ans::Var(v) = left {
            if let Some(n) = numbers.get(v) {
                *left = Ans::Num(*n);
            }
        }

        if let Ans::Var(v) = right {
            if let Some(n) = numbers.get(v) {
                *right = Ans::Num(*n);
            }
        }

        if let Some(n_var) = numbers.get(var) {
            let (op_var, pos, num) = match (left.clone(), right.clone()) {
                (Ans::Var(_v1), Ans::Var(_v2)) => {
                    index += 1;
                    continue;
                }
                (Ans::Var(v), Ans::Num(n)) => (v, true, n),
                (Ans::Num(n), Ans::Var(v)) => (v, false, n),
                (Ans::Num(_), Ans::Num(_)) => todo!(),
            };

            // println!("{n_var} {var} = {op_var}, {pos}, {num} ");

            let ans: i64 = if !pos {
                // V = NUM * R =>
                //
                match op {
                    '*' => n_var / num,
                    '+' => n_var - num,
                    '/' => num / n_var,
                    '-' => num - n_var,
                    _ => panic!("No op!"),
                }
            } else {
                match op {
                    '*' => n_var / num,
                    '+' => n_var - num,
                    '/' => n_var * num,
                    '-' => n_var + num,
                    _ => panic!("No op!"),
                }
            };

            // println!("rev: [{index}] {var} {op_var} = {ans} -- {}/{total}", numbers.len() + 1);
            if op_var == "humn" {
                break ans;
            };
            numbers.insert(op_var, ans);
            list.swap_remove(index);
            index = 0;
            continue;
        }

        if let (Ans::Num(l), Ans::Num(r)) = (left, right) {
            let ans = match op {
                '*' => *l * *r,
                '+' => *l + *r,
                '/' => *l / *r,
                '-' => *l - *r,
                _ => panic!("No op!"),
            };

            // println!("status: [{index}] {var} = {ans} -- {}/{total}", numbers.len() + 1);

            numbers.insert(var, ans);
            list.swap_remove(index);
        }

        index += 1;
    };

    // println!("humn: {ans}");

    ans
}

fn main() {
    let start_begin = Instant::now();

    let mut f = File::open("input/input.txt").unwrap();
    let mut input = String::with_capacity(4096);
    f.read_to_string(&mut input).unwrap();

    let start = Instant::now();
    let total = part1(&input);
    let end = start.elapsed();
    println!("Part1: root = {} in {} uS", total, end.as_micros());

    let start = Instant::now();

    let total = part2(&input);
    let end = start.elapsed();
    println!("Part2: humn = {} in {} uS", total, end.as_micros());

    println!("Total in {} uS", start_begin.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn example_part1() {
        let ans = part1(INPUT);
        assert_eq!(ans, 152);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(INPUT), 301);
    }
}
