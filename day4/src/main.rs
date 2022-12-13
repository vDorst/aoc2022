use std::io::Read;

fn part1(input: &[u8]) -> u32 {
    let mut total_score = 0;
    let mut total_false = 0;

    for (line, data) in input.split(|x| *x == b'\n').enumerate() {
        let pairs = parse_to_pair(data);
        let overlapp = pair_fully_overlap(&pairs);
        println!("{:?} = {overlapp:?}", pairs);
        if overlapp {
            total_score += 1
        } else {
            total_false += 1
        };
    }
    println!("total false {total_false}");
    total_score
}

fn part2(input: &[u8]) -> u32 {
    let mut total_score = 0;
    let mut total_false = 0;

    for (line, data) in input.split(|x| *x == b'\n').enumerate() {
        let pairs = parse_to_pair(data);
        let overlapp = pair_overlap(&pairs);
        println!("{:?} = {overlapp:?}", pairs);
        if overlapp {
            total_score += 1
        } else {
            total_false += 1
        };
    }
    println!("total false {total_false}");
    total_score
}

fn slice_to_number(input: &[u8]) -> u8 {
    input.iter().map(|v| *v - b'0').fold(0, |sum, x| {
        sum.checked_mul(10).unwrap().checked_add(x).unwrap()
    })
}

fn parse_to_pair(input: &[u8]) -> Vec<u8> {
    input
        .split(|v| *v == b'-' || *v == b',')
        .map(|x| slice_to_number(x))
        .collect()
}

fn main() {
    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = Vec::<u8>::with_capacity(1_000_000);
    f.read_to_end(&mut input).unwrap();

    println!("score: {}", part1(&input));
    println!("score: {}", part2(&input));
}

// [1,100,2,99]
// [2,99,1,100] 2 >= 1 = false, 1 >= 2 = true , 100 <= 99
fn pair_fully_overlap(input: &[u8]) -> bool {
    if let Some(data) = input.get(0..=3) {
        if data[0] <= data[2] && data[1] >= data[3] {
            return true;
        }
        if data[2] <= data[0] && data[3] >= data[1] {
            return true;
        }
        false
    } else {
        panic!("{input:?}");
    }
}

// [1,100,2,99]
// [2,99,1,100] 2 >= 1 = false, 1 >= 2 = true , 100 <= 99

// [2, 4, 6, 8]
fn pair_overlap(input: &[u8]) -> bool {
    if let Some(data) = input.get(0..=3) {
        if (data[0]..=data[1]).contains(&data[2]) {
            return true;
        };
        if (data[0]..=data[1]).contains(&data[3]) {
            return true;
        };
        if (data[2]..=data[3]).contains(&data[0]) {
            return true;
        };
        if (data[2]..=data[3]).contains(&data[1]) {
            return true;
        };
        false
    } else {
        panic!("{input:?}");
    }
}

#[cfg(test)]
mod tests {
    use crate::pair_overlap;

    use super::{pair_fully_overlap, parse_to_pair, part1, slice_to_number};

    const INPUT: &[u8] = b"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_slice_to_number() {
        assert_eq!(slice_to_number(b"0"), 0);
        assert_eq!(slice_to_number(b"255"), 255);
        assert_eq!(slice_to_number(b"0255"), 255);
        assert_eq!(slice_to_number(b"000"), 0);
    }

    #[test]
    fn test_slice_to_pair() {
        let mut line = INPUT.split(|v| *v == b'\n');

        let data = line.next().unwrap();
        assert_eq!(parse_to_pair(data), vec![2, 4, 6, 8]);
        let data = line.next().unwrap();
        assert_eq!(parse_to_pair(data), vec![2, 3, 4, 5]);
    }
    #[test]
    fn test_example_full_overlap() {
        assert_eq!(pair_fully_overlap(&[6, 6, 4, 6]), true);
        assert_eq!(pair_fully_overlap(&[4, 6, 6, 6]), true);
        assert_eq!(pair_fully_overlap(&[1, 100, 2, 99]), true);
        assert_eq!(pair_fully_overlap(&[2, 99, 1, 100]), true);

        assert_eq!(pair_fully_overlap(&[6, 7, 4, 6]), false);
        assert_eq!(pair_fully_overlap(&[4, 6, 6, 7]), false);
        assert_eq!(pair_fully_overlap(&[1, 98, 2, 99]), false);
        assert_eq!(pair_fully_overlap(&[2, 99, 1, 98]), false);

        assert_eq!(pair_fully_overlap(&[4, 99, 4, 47]), true);

        let ans = vec![false, false, false, true, true, false];
        let line = INPUT.split(|v| *v == b'\n');

        for (&a, l) in ans.iter().zip(line) {
            let pairs = parse_to_pair(l);
            println!("{:?}", pairs);
            assert_eq!(a, pair_fully_overlap(&pairs));
        }
    }

    #[test]
    fn test_example_overlap() {
        assert_eq!(pair_overlap(&[6, 6, 4, 6]), true);
        assert_eq!(pair_overlap(&[4, 6, 6, 6]), true);
        assert_eq!(pair_overlap(&[1, 100, 2, 99]), true);
        assert_eq!(pair_overlap(&[2, 99, 1, 100]), true);

        assert_eq!(pair_overlap(&[6, 7, 4, 6]), true);
        assert_eq!(pair_overlap(&[4, 6, 6, 7]), true);
        assert_eq!(pair_overlap(&[1, 98, 2, 99]), true);
        assert_eq!(pair_overlap(&[2, 99, 1, 98]), true);

        assert_eq!(pair_overlap(&[4, 99, 4, 47]), true);

        let ans = vec![false, false, true, true, true, true];
        let line = INPUT.split(|v| *v == b'\n');

        for (&a, l) in ans.iter().zip(line) {
            let pairs = parse_to_pair(l);
            println!("{:?}", pairs);
            assert_eq!(a, pair_overlap(&pairs));
        }
    }

    #[test]
    fn test_example_part1() {
        let ts = part1(INPUT);
        assert_eq!(ts, 2);
    }
}
