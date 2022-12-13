use std::io::Read;

fn part1(input: &[u8]) -> String {
    let mut crates = Containers {
        state: Vec::with_capacity(9),
    };

    crates.state.push(b"BZT".to_vec());
    crates.state.push(b"VHTDN".to_vec());
    crates.state.push(b"BFMD".to_vec());
    crates.state.push(b"TJGWVQM".to_vec());
    crates.state.push(b"WDGPVFQM".to_vec());
    crates.state.push(b"VZQGHFS".to_vec());
    crates.state.push(b"ZSNRLTCW".to_vec());
    crates.state.push(b"ZHWDJNRM".to_vec());
    crates.state.push(b"MQLFDS".to_vec());

    let line = input.split(|v| *v == b'\n');
    for data in line {
        println!("line: {}", core::str::from_utf8(data).unwrap());
        crates.job(Instruction::from(data))
    }

    crates.finish()
}

fn part2(input: &[u8]) -> String {
    let mut crates = Containers {
        state: Vec::with_capacity(9),
    };

    crates.state.push(b"BZT".to_vec());
    crates.state.push(b"VHTDN".to_vec());
    crates.state.push(b"BFMD".to_vec());
    crates.state.push(b"TJGWVQM".to_vec());
    crates.state.push(b"WDGPVFQM".to_vec());
    crates.state.push(b"VZQGHFS".to_vec());
    crates.state.push(b"ZSNRLTCW".to_vec());
    crates.state.push(b"ZHWDJNRM".to_vec());
    crates.state.push(b"MQLFDS".to_vec());

    let line = input.split(|v| *v == b'\n');
    for data in line {
        println!("line: {}", core::str::from_utf8(data).unwrap());
        crates.job_mutli(Instruction::from(data))
    }

    crates.finish()
}

fn main() {
    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = Vec::<u8>::with_capacity(1_000_000);
    f.read_to_end(&mut input).unwrap();

    println!("score: {}", part1(&input));
    println!("score: {}", part2(&input));
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    n: u8,
    from: u8,
    to: u8,
}

impl Instruction {
    fn from(input: &[u8]) -> Self {
        let mut line = input.split(|v| *v == b' ');

        let n = slice_to_number(line.nth(1).unwrap());
        let from = slice_to_number(line.nth(1).unwrap());
        let to = slice_to_number(line.nth(1).unwrap());

        Self { n, from, to }
    }
}

fn slice_to_number(input: &[u8]) -> u8 {
    input.iter().map(|v| *v - b'0').fold(0, |sum, x| {
        sum.checked_mul(10).unwrap().checked_add(x).unwrap()
    })
}

#[derive(Debug, PartialEq, Eq)]
struct Containers {
    state: Vec<Vec<u8>>,
}

impl Containers {
    fn job(&mut self, ins: Instruction) {
        for _ in 0..ins.n {
            let from = usize::from(ins.from) - 1;
            let krat = self.state[from].pop().unwrap();
            let to = usize::from(ins.to) - 1;
            self.state[to].push(krat);
        }
    }

    fn job_mutli(&mut self, ins: Instruction) {
        let from = usize::from(ins.from) - 1;
        let start = self.state[from].len() - usize::from(ins.n);
        let krat = self.state[from].drain(start..).as_slice().to_vec();
        let to = usize::from(ins.to) - 1;
        self.state[to].extend_from_slice(&krat);
    }

    fn finish(&self) -> String {
        let mut letters = Vec::with_capacity(self.state.len());
        for stack in &self.state {
            letters.push(*stack.last().unwrap());
        }

        String::from_utf8(letters).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::{Containers, Instruction};

    const INPUT: &[u8] = b"move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_slice_to_number() {
        let mut line = INPUT.split(|v| *v == b'\n');

        let data = line.next().unwrap();
        assert_eq!(
            Instruction::from(data),
            Instruction {
                n: 1,
                from: 2,
                to: 1
            }
        );
        let data = line.next().unwrap();
        assert_eq!(
            Instruction::from(data),
            Instruction {
                n: 3,
                from: 1,
                to: 3
            }
        );
        let data = line.next().unwrap();
        assert_eq!(
            Instruction::from(data),
            Instruction {
                n: 2,
                from: 2,
                to: 1
            }
        );
        let data = line.next().unwrap();
        assert_eq!(
            Instruction::from(data),
            Instruction {
                n: 1,
                from: 1,
                to: 2
            }
        );
        assert!(line.next().is_none());
    }

    #[test]
    fn test_example() {
        let line = INPUT.split(|v| *v == b'\n');

        let mut crates = Containers {
            state: vec![vec![b'Z', b'N'], vec![b'M', b'C', b'D'], vec![b'P']],
        };

        for data in line {
            crates.job(Instruction::from(data))
        }

        println!("{crates:?}");

        assert_eq!(
            crates,
            Containers {
                state: vec![vec![b'C'], vec![b'M'], vec![b'P', b'D', b'N', b'Z']]
            }
        );
        assert_eq!(crates.finish(), "CMZ".to_owned());
    }

    #[test]
    fn test_example_part2() {
        let mut crates = Containers {
            state: vec![vec![b'Z', b'N'], vec![b'M', b'C', b'D'], vec![b'P']],
        };

        for data in INPUT.split(|v| *v == b'\n') {
            crates.job_mutli(Instruction::from(data))
        }

        println!("{crates:?}");

        assert_eq!(
            crates,
            Containers {
                state: vec![b"M".to_vec(), vec![b'C'], b"PZND".to_vec()]
            }
        );
        assert_eq!(crates.finish(), "MCD".to_owned());
    }
}
