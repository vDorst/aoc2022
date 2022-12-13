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
    f.read_to_end(&mut input).unwrap();

    let mut score_total: u32 = 0;

    let mut rounds = 0;

    for data in input.chunks(4) {
        rounds += 1;
        let move_opponent = PRS::from(data[0]);
        let sc = Score::from(data[2]);
        let move_wants = PRS::from(data[2]);
        //let move_me = move_opponent.strategy_play();
        let move_me = move_opponent.known_score(sc);
        let score_won = Score::play(move_opponent, move_me);
        let score_round = score_won as u8 + move_me.clone() as u8;
        println!("{rounds}: Opponent: {move_opponent:?} Me: {move_wants:?} plays {move_me:?}: {score_won:?} ( {} + {} ) = {score_round}", move_me.clone() as u8, score_won as u8);

        score_total += u32::from(score_round);
    }

    println!("Total score: {score_total}");
}

#[derive(Copy, Clone)]
#[repr(u8)]

enum PRS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl std::fmt::Debug for PRS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rock => write!(f, "Rock    "),
            Self::Paper => write!(f, "Paper   "),
            Self::Scissors => write!(f, "Scissors"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Score {
    Lost = 0,
    Draw = 3,
    Won = 6,
}

impl Score {
    fn from(inp: u8) -> Self {
        match inp {
            b'A' | b'X' => Self::Lost,
            b'B' | b'Y' => Self::Draw,
            b'C' | b'Z' => Self::Won,
            _ => panic!("inp {inp}"),
        }
    }
}

impl PRS {
    fn from(inp: u8) -> Self {
        match inp {
            b'A' | b'X' => Self::Rock,
            b'B' | b'Y' => Self::Paper,
            b'C' | b'Z' => Self::Scissors,
            _ => panic!("inp {inp}"),
        }
    }

    fn strategy_play(&self) -> Self {
        match self {
            &PRS::Rock => Self::Paper,
            &PRS::Paper => Self::Rock,
            &PRS::Scissors => Self::Scissors,
        }
    }
    fn known_score(&self, score: Score) -> Self {
        match (self, score) {
            (PRS::Rock, Score::Lost) => Self::Scissors,
            (PRS::Rock, Score::Draw) => Self::Rock,
            (PRS::Rock, Score::Won) => Self::Paper,
            (PRS::Paper, Score::Lost) => Self::Rock,
            (PRS::Paper, Score::Draw) => Self::Paper,
            (PRS::Paper, Score::Won) => Self::Scissors,
            (PRS::Scissors, Score::Lost) => Self::Paper,
            (PRS::Scissors, Score::Draw) => Self::Scissors,
            (PRS::Scissors, Score::Won) => Self::Rock,
        }
    }
}

impl Score {
    fn play(oponent: PRS, myself: PRS) -> Self {
        match (oponent, myself) {
            (PRS::Rock, PRS::Rock) => Self::Draw,
            (PRS::Rock, PRS::Paper) => Self::Won,
            (PRS::Rock, PRS::Scissors) => Self::Lost,
            (PRS::Paper, PRS::Rock) => Self::Lost,
            (PRS::Paper, PRS::Paper) => Self::Draw,
            (PRS::Paper, PRS::Scissors) => Self::Won,
            (PRS::Scissors, PRS::Rock) => Self::Won,
            (PRS::Scissors, PRS::Paper) => Self::Lost,
            (PRS::Scissors, PRS::Scissors) => Self::Draw,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Score, PRS};

    const INPUT: &[u8] = b"A Y\nB X\nC Z\n";

    #[test]
    fn test_example_part1() {
        let mut score_total: u32 = 0;
        let ans_score = [8, 1, 6];

        for (data, ans) in INPUT.chunks(4).zip(ans_score) {
            let move_opponent = PRS::from(data[0]);
            let move_me = move_opponent.strategy_play();
            let mut score_round = move_me.clone() as u8;
            let score_won = Score::play(move_opponent, move_me);
            score_round += score_won as u8;
            assert_eq!(score_round, ans);
            score_total += u32::from(score_round);
        }
        assert_eq!(score_total, 15);
    }

    #[test]
    fn test_example_part2() {
        let mut score_total: u32 = 0;
        let ans_score = [Score::Draw, Score::Lost, Score::Won];

        for (data, ans) in INPUT.chunks(4).zip(ans_score) {
            let move_opponent = PRS::from(data[0]);
            let outcome = Score::from(data[2]);
            let move_me = move_opponent.known_score(outcome);
            let mut score_round = move_me.clone() as u8;
            let score_won = Score::play(move_opponent, move_me);
            score_round += score_won as u8;
            assert_eq!(score_won, ans);
            score_total += u32::from(score_round);
        }
        assert_eq!(score_total, 12);
    }
}
