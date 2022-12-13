use std::io::Read;

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    let mut high = 0;
    for line in input.split_terminator('\n') {
        if let Ok(num) = line.parse::<u32>() {
            sum += num;
        } else {
            high = high.max(sum);
            sum = 0;
        }
    }
    high = high.max(sum);

    high
}

struct High([u32; 3]);

impl High {
    fn insert(&mut self, val: u32) {
        // println!("\tcheck {val}");

        if val > self.0[2] {
            for n in (0..=2).rev() {
                self.0[n] = if n != 0 { val.min(self.0[n - 1]) } else { val };
                // println!("val {n} {}", self.0[n]);
                if self.0[n] == val {
                    //println!("insert in {n}");
                    break;
                }
            }
        }
    }

    fn sum(&self) -> u32 {
        self.0[0] + self.0[1] + self.0[2]
    }
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;

    let mut high = High([0, 0, 0]);

    for line in input.split_terminator('\n') {
        if let Ok(num) = line.parse::<u32>() {
            sum += num;
        } else {
            high.insert(sum);
            sum = 0;
        }
    }
    high.insert(sum);

    high.sum()
}

fn main() {
    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = Vec::<u8>::with_capacity(1_000_000);

    f.read_to_end(&mut input);

    let s = String::from_utf8(input).unwrap();

    let total = part1(&s);

    println!("cal: {total}");

    let total = part2(&s);

    println!("cal: {total}");
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn test_example_part1() {
        assert_eq!(super::part1(INPUT), 24000);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(super::part2(INPUT), 45000);
    }
}
