use std::{
    fmt::Display,
    io::Read,
    ops::{Add, AddAssign},
};

const INPUT: &[u8] = b"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

type SnafuValue = i64;

#[derive(Debug)]
struct SNAFU(SnafuValue);

impl Add for SNAFU {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for SNAFU {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl PartialEq<SnafuValue> for SNAFU {
    fn eq(&self, other: &SnafuValue) -> bool {
        self.0 == *other
    }
}

impl SNAFU {
    fn from(number: &[u8]) -> Self {
        let mut num = 0;

        for (n, v) in number.iter().rev().enumerate() {
            let number: SnafuValue = match v {
                b'-' => -1,
                b'=' => -2,
                o => SnafuValue::from(o - b'0'),
            };
            let power_of_five = SnafuValue::try_from(SnafuValue::from(5).pow(n as u32)).unwrap();

            println!(
                "{} * {n}^5 = {number} * {power_of_five} = {} ",
                char::from(*v),
                number * power_of_five
            );
            num += number * power_of_five;
        }

        println!("Sum = {num}");
        Self(num)
    }

    fn to_snafu(&self) -> String {
        let mut snafu = String::with_capacity(10);
        let mut num = self.0;

        //println!("to_snafu: {num}");

        let abs = num.abs();
        if abs > 2_i64.pow(52) { panic!("Number to high {num}")}

        let abs_f = abs as f64;

        let p_f = abs_f.log10() / 5.0_f64.log10();

        let power_of_five_num = if num.abs_diff(0) <= 2  { 0 } else { p_f.round() as u32};

        //println!("\t--: {num} {abs} P{power_of_five_num} abs:{abs_f} p: {p_f}");

        for pow in (0..=power_of_five_num).rev() {
            let power_of_five = SnafuValue::try_from(5_i64.pow(pow)).unwrap();

            let div = if num.abs_diff(0) <= 2 { num } else { (num as f64 / power_of_five as f64).round() as i64 };

            if div == 0 && pow != 0 && snafu.is_empty() {  continue; }

            // print!("{pow}: D{div} * {power_of_five} N{num}");

            snafu.push( match div {
                0 => '0',
                1 => '1',
                2 => '2',
                -1 => '-',
                -2 => '=',
                _ => '?',
            });

            num -= power_of_five * div;

            //println!(" -> {num}");


            if pow == 0 { break }
        }

        snafu
    }
}

fn part1(input: &[u8]) -> SNAFU {
    let mut sum = SNAFU(0);
    for val in input.split(|v| *v == b'\n') {
        sum += SNAFU::from(val);
    }
    sum
}

fn main() {
    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = Vec::<u8>::with_capacity(1_000_000);
    f.read_to_end(&mut input).unwrap();

    let number = part1(&input);

    println!("number: {} | {}", number.0, number.to_snafu());
}

#[cfg(test)]

mod tests {

    use super::{SnafuValue, INPUT, SNAFU};

    const INPUT_TEST: &[(&[u8], SnafuValue)] = &[
        (b"1=-0-2", 1747),
        (b"12111", 906),
        (b"2=0=", 198),
        (b"21", 11),
        (b"2=01", 201),
        (b"111", 31),
        (b"20012", 1257),
        (b"112", 32),
        (b"1=-1=", 353),
        (b"1-12", 107),
        (b"12", 7),
        (b"1=", 3),
        (b"122", 37),
    ];

    const INPUT_DECIMAL_TO_SNAFU: &[(SnafuValue, &[u8])] = &[
        (0, b"0"),
        (1, b"1"),
        (2, b"2"),
        (3, b"1="),
        (4, b"1-"),
        (5, b"10"),
        (6, b"11"),
        (7, b"12"),
        (8, b"2="),
        (9, b"2-"),
        (10, b"20"),
        (15, b"1=0"),
        (20, b"1-0"),
        (2022, b"1=11-2"),
        (12345, b"1-0---0"),
        (314159265, b"1121-1110-1=0"),
    ];

    #[test]
    fn test_snafu_to_decimal() {
        let mut sum = SNAFU(0);
        for val in INPUT_TEST {
            let num = SNAFU::from(val.0);
            assert_eq!(num, val.1);
            sum += num;
        }
        assert_eq!(sum, 4890);

        for val in INPUT_DECIMAL_TO_SNAFU {
            let num = SNAFU::from(val.1);
            assert_eq!(num, val.0);
        }
    }

    #[test]
    fn test_decimal_to_snafu() {
        for val in INPUT_DECIMAL_TO_SNAFU {
            let num = SNAFU(val.0);
            assert_eq!(num.to_snafu(), String::from_utf8_lossy(val.1));
        }
        assert_eq!(SNAFU(4890).to_snafu(), "2=-1=0");
    }
}
