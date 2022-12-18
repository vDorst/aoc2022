use std::{io::Read, time::Instant};

const FIELD_WIDTH: usize = 7;
const FIELD_HEIGHT: usize = 4096;

mod pos {
    pub const EMPTY: u8 = b'.';
    pub const BLOCK: u8 = b'#';
    pub const SAND: u8 = b'O';
    pub const START: u8 = b'+';
}
struct Timer(Instant);

impl Timer {
    fn new() -> Self {
        Self(std::time::Instant::now())
    }
    fn update(&mut self, data: &str) {
        let elapsed = self.0.elapsed();
        println!("{data}: {} uS", elapsed.as_micros());
        self.0 = std::time::Instant::now();
    }
}

fn main() {
    let mut total = Timer::new();
    let mut timer = Timer::new();

    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = Vec::<u8>::with_capacity(1_000_000);

    f.read_to_end(&mut input).unwrap();

    timer.update("Load data");

    let mut sim = PlayField::new(input.as_slice());

    sim.sim(2022);
    println!("Units: {}", sim.max());

    timer.update("Part1");

    //println!("num: {cnt}");

    total.update("Total");
}

enum Shapes {
    Min,
    Plus,
    RightAngle,
    Pipe,
    Block,
}

impl Shapes {
    fn from(blocks: u32) -> Self {
        match blocks % 5 {
            0 => Shapes::Min,
            1 => Shapes::Plus,
            2 => Shapes::RightAngle,
            3 => Shapes::Pipe,
            4 => Shapes::Block,
            e => panic!("Unknown block {e}"),
        }
    }

    fn shape(&self) -> Shape {
        match self {
            Shapes::Min => Shape(vec![(0, 1), (0, 1), (0, 1), (0, 1)]),
            Shapes::Plus => Shape(vec![(1, 1), (0, 3), (1, 1)]),
            Shapes::RightAngle => Shape(vec![(0, 1), (0, 1), (0, 3)]),
            Shapes::Pipe => Shape(vec![(0, 4)]),
            Shapes::Block => Shape(vec![(0, 2), (0, 2)]),
        }
    }
}

struct Shape(Vec<(u8, u8)>);

struct PlayField<'a> {
    input: &'a [u8],
    step: usize,
    field: [[u8; FIELD_HEIGHT]; FIELD_WIDTH],
    blocks: u32,
    height: usize,
}

impl<'a> PlayField<'a> {
    fn new(input: &'a [u8]) -> Self {
        //let x = [EMPTY; 4098];
        Self {
            input,
            step: 0,
            // field: [
            //     x.clone(),
            //     x.clone(),
            //     x.clone(),
            //     x.clone(),
            //     x.clone(),
            //     x.clone(),
            //     x.clone(),
            // ],
            field: [[pos::EMPTY; FIELD_HEIGHT]; FIELD_WIDTH],
            blocks: 0,
            height: 0,
        }
    }

    fn colom_height(&self, n: usize) -> usize {
        for pos in (0..self.height).rev() {
            if self.field[n][pos] == pos::BLOCK {
                return pos + 1;
            }
        }
        0
    }

    fn len_map(&self) -> Vec<usize> {
        let mut len = Vec::<usize>::with_capacity(FIELD_WIDTH);
        for n in 0..FIELD_WIDTH {
            len.push(self.colom_height(n));
        }
        len
    }

    fn min(&self) -> usize {
        1
    }

    fn max(&self) -> usize {
        self.height
    }

    fn draw(&self) {
        let min = self.min();
        let max = self.max();

        for line in (min ..=max + 10).rev() {
            print!("|");
            for h in &self.field {
                let c = if let Some(c) = h.get(line - 1) {
                    *c
                } else {
                    b' '
                };
                print!("{}", char::from(c));
            }
            println!("| {line}");
        }
        if min == 1 {
            println!("+-------+");
        }
    }

    fn sim(&mut self, blocks: u32) {
        let mut shape = Shapes::from(self.blocks).shape();
        let mut x = 2;
        let mut max = self.height;
        let mut row = max + 4;

        loop {
            // Ended?
            let mut new_block = row == 0;
            //if max >= row
            if !new_block
            {
                for (egde, top) in shape.0.iter().zip(&self.field[x..]) {
                    // println!("--- Step: {} Top {}, Row {row} edge ({},{}) ", self.step, top, egde.0, egde.1);
                    //if row + usize::from(egde.0) <= top.len() {
                    if let Some(&pos::BLOCK) = top.get(row + usize::from(egde.0) - 1) {
                        new_block = true;
                        break;
                    }
                }
            }

            //println!("#### Step: {}, Row: {row}, X {x}, blocks {}, new_block {}", self.step, self.blocks, new_block);

            if new_block {
                for (egde, top) in shape.0.iter().zip(&mut self.field[x..]) {
                    for n in egde.0..egde.0+egde.1 {
                        top[row + usize::from(n) ] = b'#';
                        self.height = self.height.max(row + usize::from(n) + 1);
                    }
                    // println!("Step: {} Top {}, Row {row} edge ({},{}) ", self.step, *top, egde.0, egde.1);
                }
                self.blocks += 1;
                // Done?
                if self.blocks == blocks {
                    break;
                }

                // Next block
                shape = Shapes::from(self.blocks).shape();
                x = 2;
                max = self.height;
                row = max + 4;
            }

            // Get direction
            let dir = self.input[self.step];

            let new_x: Option<usize> = match dir {
                b'<' => {
                    if x != 0 {
                        Some(x - 1)
                    } else {
                        None
                    }
                }
                b'>' => {
                    if x + shape.0.len() < FIELD_WIDTH {
                        Some(x + 1)
                    } else {
                        None
                    }
                }
                e => panic!("Unknown Move {e}"),
            };

            print!("\tCan move X {x} -> {new_x:?}: ");

            if let Some(new_x) = new_x {
                let mut can_move = true;

                // if shape
                //     .0
                //     .iter()
                //     .zip(&self.field[new_x..])
                //     .any(|(bottom_edge, stack_top)| {
                //         row + usize::from(bottom_edge.0) >= stack_top.len()
                //     })
                // {}
                // Can maybe move
                for (edge, top) in shape.0.iter().zip(&self.field[new_x..]) {
                    for n in edge.0..edge.0 + edge.1 {
                        if let Some(b'#') = top.get(row + usize::from(n) - 1) {
                            can_move = false;
                            break;
                        }
                    }
                }
                

                if can_move {
                    println!("Moved");
                    x = new_x
                } else {
                    println!("Block");
                }
            } else {
                println!("Edge!");
            }

            // Next row
            row -= 1;
            self.step += 1;
            if self.step == self.input.len() {
                self.step = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PlayField;

    const INPUT: &[u8] = b">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_draw_example() {
        let mut sim = PlayField::new(INPUT);

        sim.sim(1);

        sim.draw();

        assert_eq!(sim.len_map(), [0, 0, 1, 1, 1, 1, 0]);

        sim.sim(2);

        sim.draw();

        assert_eq!(sim.len_map(), [0, 0, 3, 4, 3, 1, 0]);

        sim.sim(3);

        sim.draw();

        assert_eq!(sim.len_map(), [4, 4, 6, 4, 3, 1, 0]);

        sim.sim(8);

        sim.draw();

        assert_eq!(sim.len_map(), [4, 12, 13, 13, 13, 15, 0]);

        sim.sim(9);

        sim.draw();

        assert_eq!(sim.len_map(), [4, 12, 13, 13, 17, 15, 0]);

        sim.sim(10);

        sim.draw();

        assert_eq!(sim.len_map(), [14, 14, 13, 13, 17, 15, 0]);
    }

    #[test]
    fn test_example() {
        let mut sim = PlayField::new(INPUT);

        sim.sim(2022);

        sim.draw();

        assert_eq!(sim.max(), 3068);
    }

    #[test]
    fn test_shift_fit() {
        let mut sim = PlayField::new(b"<<<<>>");

        sim.field[5][1] = b'#';
        sim.field[6][0] = b'#';
        sim.field[6][1] = b'#';

        sim.height = 2;

        assert_eq!(sim.len_map(), [0, 0, 0, 0, 0, 2, 2]);

        sim.sim(1);

        sim.draw();

        assert_eq!(sim.max(), 2);

        assert_eq!(sim.len_map(), [0, 0, 1, 1, 1, 2, 2]);
    }
}
