use std::io::Read;

fn part1(input: &[u8]) -> usize {
    let mut line = input.split(|v| *v == b'\n');

    let data = line.next().unwrap();
    detect_sop(data, 4)
}

fn part2(input: &[u8]) -> usize {
    let mut line = input.split(|v| *v == b'\n');

    let data = line.next().unwrap();
    detect_sop(data, 14)
}

fn main() {
    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = Vec::<u8>::with_capacity(1_000_000);
    f.read_to_end(&mut input).unwrap();

    println!("score: {}", part1(&input));
    println!("score: {}", part2(&input));
}

fn detect_sop(input: &[u8], n: usize) -> usize {
    println!("");
    let mut buf = vec![0_u8; n - 1];

    let mut rep = n;

    for pos in 0..input.len() {
        let c = input[pos];

        // for p in 0..buf.len() {
        //     let b = buf[ n - 1 - ((p + k + 1) % n) ];
        //     if b == c {
        //         rep = ( n - p ).max(rep);
        //         println!("\t pos {p}, set rep {rep}");
        //     }
        // }

        for (p, b) in buf.iter().enumerate() {
            if *b == c {
                rep = (p + 2).max(rep);
                println!("\t pos {p}, set rep {rep}");
            }
        }

        println!("{buf:02X?}, {c:02x} {} r{rep} p{pos}", char::from(c));

        buf.remove(0);
        buf.push(c);

        rep -= 1;

        if rep == 0 {
            return pos + 1;
        }
    }
    panic!("no pos!");
}

#[cfg(test)]
mod tests {
    use super::detect_sop;

    const INPUT: &[u8] = b"mjqjpqmgbljsphdztnvjfqwrcgsmlb\nbvwbjplbgvbhsrlpgdmjqwftvncz\nnppdvjthqldpwncqszvftbrmjlhg\nnznrnfrfntjfmvfwmzdfjlvtqnbhcprsg\nzcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_detect_sop() {
        let mut line = INPUT.split(|v| *v == b'\n');

        let data = line.next().unwrap();
        assert_eq!(detect_sop(data, 4), 7);

        let data = line.next().unwrap();
        assert_eq!(detect_sop(data, 4), 5);
        let data = line.next().unwrap();
        assert_eq!(detect_sop(data, 4), 6);
        let data = line.next().unwrap();
        assert_eq!(detect_sop(data, 4), 10);
        let data = line.next().unwrap();
        assert_eq!(detect_sop(data, 4), 11);
        assert!(line.next().is_none());
    }

    #[test]
    fn test_detect_sop_part2() {
        let mut line = INPUT.split(|v| *v == b'\n');

        let data = line.next().unwrap();
        assert_eq!(detect_sop(data, 14), 19);
    }
}
