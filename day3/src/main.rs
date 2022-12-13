use std::io::Read;

fn slice_to_bin(input: &[u8]) -> u64 {
    input
        .iter()
        .map(|v| if *v >= b'a' { v - b'a' } else { v - b'A' + 26 })
        .fold(0, |sum, x| sum | (1 << u64::from(x)))
}

fn part1(input: &[u8]) -> u32 {
    let mut total_score = 0;

    for (line, data) in input.split(|x| *x == b'\n').enumerate() {
        // assert!(data.is_empty());
        // assert!(data.len() & 1 == 0);
        let (c1, c2) = data.split_at(data.len() / 2);
        let c1 = slice_to_bin(c1);
        let c2 = slice_to_bin(c2);
        let common = c1 & c2;
        let score = 1 + common.trailing_zeros();
        total_score += score;
        println!(
            "{line}: {} {common:8x} score {score}",
            core::str::from_utf8(data).unwrap()
        );
    }
    total_score
}

fn part2(input: &[u8]) -> u32 {
    let mut total_score = 0;
    let mut line = 0;

    let mut data = input.split(|x| *x == b'\n');

    'lus: loop {
        let mut common = u64::MAX;

        for _ in 0..3 {
            if let Some(d) = data.next() {
                common &= slice_to_bin(d);
            } else {
                break 'lus;
            }
        }

        let score = 1 + common.trailing_zeros();
        total_score += score;
        println!("{line}: {common:8x} score {score}");
        line += 1;
    }
    total_score
}

fn main() {
    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = Vec::<u8>::with_capacity(1_000_000);
    f.read_to_end(&mut input).unwrap();

    //println!("score: {}", part1(&input));

    println!("score: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::{part1, slice_to_bin};

    const INPUT: &[u8] = b"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_slice_to_bin() {
        assert_eq!(slice_to_bin(b"a"), 1 << 0);
        assert_eq!(slice_to_bin(b"z"), 1 << 25);
        assert_eq!(slice_to_bin(b"A"), 1 << 26);
        assert_eq!(slice_to_bin(b"Z"), 1 << 51);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(b"aa"), 1);
        assert_eq!(part1(b"zz"), 26);
        assert_eq!(part1(b"AA"), 27);
        assert_eq!(part1(b"ZZ"), 52);
    }

    #[test]
    fn test_example_part1() {
        let ts = part1(INPUT);
        assert_eq!(ts, 157);
    }
}
