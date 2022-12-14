#![deny(clippy::pedantic)]

use core::str::from_utf8;

type Signal = i16;

type X = i8;

#[derive(Debug, PartialEq, Eq)]

enum Op {
    Noop,

    Addx(X),
}

struct VideoSystem {
    cycle: u8,

    x: X,

    crt: [u8; 240],
}

impl VideoSystem {
    const WIDTH: usize = 40;

    fn new() -> Self {
        Self {
            cycle: 0,

            x: 1,

            crt: [b' '; 240],
        }
    }

    fn cycle_check(&self) -> Option<Signal> {
        if [20_u8, 60, 100, 140, 180, 220].contains(&self.cycle) {
            return Some(Signal::from(self.cycle) * Signal::from(self.x));
        }

        None
    }

    fn draw_pixel(&mut self) {
        let mem = usize::from(self.cycle - 1);

        if mem < self.crt.len() {
            let x = mem % Self::WIDTH;

            self.crt[mem] = if X::try_from(x).unwrap().abs_diff(self.x) <= 1 {
                b'#'
            } else {
                b'.'
            };
        }
    }

    fn draw_screen(&self) {
        for line in self.crt.chunks_exact(Self::WIDTH) {
            println!("{}", from_utf8(line).unwrap());
        }
    }

    fn clock_tick(&mut self) {
        self.cycle += 1;

        self.draw_pixel();
    }

    fn instruction(&mut self, state: &Op) -> Option<Signal> {
        self.clock_tick();

        let mut signal = self.cycle_check();

        match *state {
            Op::Noop => (),

            Op::Addx(v) => {
                self.clock_tick();

                if signal.is_none() {
                    signal = self.cycle_check();
                }

                self.x += v;
            }
        }

        signal
    }
}

fn decode(instr: &[u8]) -> Op {
    // println!("{}", from_utf8(instr).unwrap());

    if instr == b"noop" {
        return Op::Noop;
    }

    if let (b"addx ", v) = instr.split_at(5) {
        let v = from_utf8(v).unwrap().parse::<X>().unwrap();

        return Op::Addx(v);
    }

    panic!("Unknown instruction: {}", from_utf8(instr).unwrap());
}

fn main() {
    let input = include_bytes!("../input/input.txt");

    let mut vs = VideoSystem::new();

    let mut signal = 0;

    for line in input.split(|v| *v == b'\n') {
        if line.is_empty() {
            break;
        }

        let instr = decode(line);

        if let Some(v) = vs.instruction(&instr) {
            signal += v;
        }
    }

    println!("Signal: {signal}");

    vs.draw_screen();
}

#[cfg(test)]

mod tests {

    use crate::Op;

    use super::{
        decode,
        Op::{Addx, Noop},
        VideoSystem,
    };

    const INPUT: &[u8] = b"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    const CRT_ANS: &[u8] = b"##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....";

    #[test]

    fn test_vecvec_new() {
        let mut cpu = VideoSystem::new();

        assert_eq!(cpu.instruction(&Noop), None);

        assert_eq!(cpu.instruction(&Addx(3)), None);

        assert_eq!(cpu.x, 4);

        assert_eq!(cpu.instruction(&Addx(-5)), None);

        assert_eq!(cpu.x, -1);
    }

    #[test]

    fn test_decode() {
        assert_eq!(decode(b"noop"), Op::Noop);

        assert_eq!(decode(b"addx 0"), Op::Addx(0));

        assert_eq!(decode(b"addx -15"), Op::Addx(-15));

        assert_eq!(decode(b"addx 1"), Op::Addx(1));
    }

    #[test]

    fn test_example() {
        let mut cpu = VideoSystem::new();

        let mut signal = 0;

        for line in INPUT.split(|v| *v == b'\n') {
            let instr = decode(line);

            if let Some(v) = cpu.instruction(&instr) {
                signal += v;
            }
        }

        assert_eq!(signal, 13140);

        cpu.draw_screen();

        assert_eq!(&cpu.crt, CRT_ANS);
    }
}
