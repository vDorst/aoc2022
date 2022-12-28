use std::{fmt::Display, io::Read};

fn part1(input: &str) -> isize {

    let dec = Grove::new(input);

    let ret = dec.process(1);

    let total = ret.1.iter().map(|v| *v as isize).sum();

    println!("{:?}: {total}", ret.1);

    total
}

fn part2(input: &str) -> Gps {
    let mut dec = Grove::new(input);
    dec.set_key(811589153);

    let ret = dec.process(10);

    let total = ret.1.iter().sum();

    println!("{:?}: {total}", ret.1);

    total
}

fn main() {
    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = String::with_capacity(1_000_000);
    f.read_to_string(&mut input).unwrap();

    let answer = part1(&input);
    println!("Part1: sum: {}", answer);
    // println!("Part2: sum: {}", part2(&input));

    let answer = part2(&input);
    println!("Part2: sum: {}", answer);
}

const INPUT: &str = "1\n2\n-3\n3\n-2\n0\n4";

type Gps = i64;
type Idx = u16;

struct Number {
    pos: u16,
    value: Gps,
}

struct Grove(Vec<Gps>);

impl Grove {
    fn new(input: &str) -> Self {
        Self(input.split('\n').into_iter().map(|v| v.parse::<Gps>().unwrap()).collect() )
    }

    fn set_key(&mut self, key: Gps) {
        for v in self.0.iter_mut() {
            *v *= key;
        }
    }

    fn insert_pos(&self, idx: Idx, value: Gps) -> usize {
        let val = value as isize;
        let idx = idx as isize;

        let len = self.0.len() as isize;

        let mut ipos = idx + val;

        ipos = ipos.rem_euclid(len - 1);

        if val != 0 && idx + val == 0 {
            ipos = len - 1;
        }

        // println!("{val} {idx} {ipos}");
 
        ipos as usize
    }

    fn process(&self, times: u8) -> (Vec<Idx>, Vec<Gps>) {
        let idx_len = Idx::try_from(self.0.len()).unwrap();
        let mut list: Vec<Idx> = (0..idx_len).collect();

        for _ in 0..times {
            for (idx, &val) in self.0.iter().enumerate() {
                let list_idx = list.iter().position(|v| *v == idx as Idx ).unwrap() as Idx;

                let ins_idx = self.insert_pos(list_idx, val);

                let item = list.remove(list_idx as usize);

                list.insert(ins_idx, item);
            }
        }

        let idx_zero = self.0.iter().position(|v| *v == 0 ).unwrap();
        let idx_zero = list.iter().position(|v| *v == idx_zero as u16 ).unwrap();

        let mut cor = Vec::<Gps>::with_capacity(3);
        for n in 1..=3 {
            let idx = (idx_zero + (n * 1000)) % self.0.len();
            let idx = list[idx] as usize;
            cor.push( self.0[idx] );
        }

        (list, cor)
    }
    

}


#[cfg(test)]
mod tests {
    use super::{INPUT, Grove, Gps, Read};

    #[test]
    fn test_example1() {
        let dec = Grove::new(INPUT);

        let ret = dec.process(1);

        let num: Vec<Gps> = ret.0.iter().map(|n| dec.0[*n as usize]).collect();

        assert_eq!(num, vec![1, 2, -3, 4, 0, 3, -2]);
        assert_eq!(ret.1, vec![4, -3, 2])
    }

    #[test]
    fn test_example2() {
        let mut dec = Grove::new(INPUT);

        dec.set_key(811589153);

        assert_eq!(dec.0, vec![811589153, 1623178306, -2434767459, 2434767459, -1623178306, 0, 3246356612]);

        let ret = dec.process(10);

        let num: Vec<Gps> = ret.0.iter().map(|n| dec.0[*n as usize]).collect();

        assert_eq!(num, vec![0, -2434767459, 1623178306, 3246356612, -1623178306, 2434767459, 811589153]);
        assert_eq!(ret.1, vec![811589153, 2434767459, -1623178306])
    }

    #[test]
    fn test_example_go() {
        let mut f = std::fs::File::open("input/input_test.txt").unwrap();
        let mut input = String::with_capacity(1_000_000);
        f.read_to_string(&mut input).unwrap();

        let dec = Grove::new(&input);

        let ret = dec.process(1);

        let answer: isize = ret.1.iter().map(|v| *v as isize).sum();

        println!("Part1: sum: {}", answer);
        assert_eq!(answer, 3466);
    }

    #[test]
    fn test_example_part1() {
        let mut f = std::fs::File::open("input/input.txt").unwrap();
        let mut input = String::with_capacity(1_000_000);
        f.read_to_string(&mut input).unwrap();

        let dec = Grove::new(&input);

        let ret = dec.process(1);

        let answer: isize = ret.1.iter().map(|v| *v as isize).sum();

        println!("Part1: sum: {}", answer);
        assert_eq!(answer, 4914);
    }

   
}
