use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = Vec::<u8>::with_capacity(1_000_000);
    f.read_to_end(&mut input).unwrap();

    let map = Map::new(&input);

    let dir = map.create_direction_map();

    let total = dir.iter().fold(0, |acc, f| acc + usize::from(f.0 != 0));

    println!("total: {total}");

    let scienic = map.create_scenic_map();

    println!(
        "Best: {} Sum: {}",
        scienic.iter().max().unwrap(),
        scienic.iter().sum::<usize>()
    );
}

#[repr(u8)]
enum Side {
    Top = 1,
    Right = 2,
    Bottom = 4,
    Left = 8,
}

#[derive(Debug, PartialEq, Eq)]
struct Visable(u8);

impl Visable {
    fn new() -> Self {
        Self(0)
    }
    fn Top(&mut self) {
        self.0 |= Side::Top as u8;
    }
    fn Right(&mut self) {
        self.0 |= Side::Right as u8;
    }
    fn Bottom(&mut self) {
        self.0 |= Side::Bottom as u8;
    }
    fn Left(&mut self) {
        self.0 |= Side::Left as u8;
    }
}

struct Map {
    x: usize,
    y: usize,
    data: Vec<u8>,
}

impl Map {
    fn new(data: &[u8]) -> Self {
        let mut line = data.split(|v| *v == b'\n');

        let first_row = line.next().unwrap();
        let mut map = Map {
            x: first_row.len(),
            y: 1,
            data: Vec::with_capacity(2000),
        };
        map.data.extend_from_slice(first_row);
        for data in line {
            if data.len() != map.x {
                panic!("data.len() != {}", map.x);
            }
            map.data.extend_from_slice(data);
            map.y += 1;
        }

        map
    }

    fn visable(&self, n: usize) -> Visable {
        let row_x = n % self.x;
        let row_y = n / self.x;

        let mut vis = Visable::new();

        let is_left = row_x == 0;
        let is_right = row_x == (self.x - 1);
        let is_top = row_y == 0;
        let is_bottum = row_y == (self.y - 1);

        let tree = self.data[n] - b'0';

        if is_left {
            vis.Left()
        };
        if is_right {
            vis.Right()
        }
        if is_top {
            vis.Top()
        };
        if is_bottum {
            vis.Bottom()
        }

        if !is_right {
            let s = n + 1;
            let e = self.x * (row_y + 1);
            let data = &self.data[s..e];
            let max = data.iter().map(|x| x - b'0').max().unwrap();

            //println!("{tree}: {} {max}", core::str::from_utf8(data).unwrap());
            if tree > max {
                vis.Right();
            }
        }
        if !is_left {
            let e = n;
            let s = self.x * row_y;
            let data = &self.data[s..e];
            let max = data.iter().map(|x| x - b'0').max().unwrap();

            //println!("{tree}: {} {max}", core::str::from_utf8(data).unwrap());
            if tree > max {
                vis.Left();
            }
        }

        if !is_top {
            let mut e = n - self.x;
            let mut max = 0;
            loop {
                let d = self.data.get(e).unwrap();
                max = max.max(d - b'0');
                if e < self.x {
                    break;
                }
                e -= self.x;
            }

            // println!("{tree}: {} {max}", core::str::from_utf8(data).unwrap());
            if tree > max {
                vis.Top();
            }
        }

        if !is_bottum {
            let mut e = n + self.x;
            let mut max = 0;
            while let Some(&d) = self.data.get(e) {
                max = max.max(d - b'0');
                e += self.x;
            }

            // println!("{tree}: {} {max}", core::str::from_utf8(data).unwrap());
            if tree > max {
                vis.Bottom();
            }
        }

        vis
    }

    fn tree_score(&self, n: usize) -> usize {
        let row_x = n % self.x;
        let row_y = n / self.x;

        let mut score: usize = 1;

        let is_left = row_x == 0;
        let is_right = row_x == (self.x - 1);
        let is_top = row_y == 0;
        let is_bottum = row_y == (self.y - 1);

        let tree = self.data[n] - b'0';

        // if is_left {
        //     score.Left()
        // };
        // if is_right {
        //     score.Right()
        // }
        // if is_top {
        //     score.Top()
        // };
        // if is_bottum {
        //     score.Bottom()
        // }

        if !is_right {
            let s = n + 1;
            let e = self.x * (row_y + 1);
            let data = &self.data[s..e];

            let mut distance = 0;
            for d in data {
                distance += 1;

                if d - b'0' >= tree {
                    break;
                }
            }
            score *= distance;
        }
        if !is_left {
            let e = n;
            let s = self.x * row_y;
            let data = &self.data[s..e];

            let mut distance = 0;
            for d in data.iter().rev() {
                distance += 1;
                if d - b'0' >= tree {
                    break;
                }
            }
            score *= distance;
        }

        if !is_top {
            let mut e = n - self.x;
            let mut distance = 0;
            loop {
                let &d = self.data.get(e).unwrap();
                distance += 1;
                if d - b'0' >= tree {
                    break;
                }
                if e < self.x {
                    break;
                }

                e -= self.x;
            }
            score *= distance;
        }

        if !is_bottum {
            let mut e = n + self.x;
            let mut distance = 0;
            while let Some(&d) = self.data.get(e) {
                distance += 1;
                if d - b'0' >= tree {
                    break;
                }
                e += self.x;
            }
            score *= distance;
        }

        score
    }

    fn create_direction_map(&self) -> Vec<Visable> {
        let mut vis = Vec::with_capacity(self.data.len());

        for n in 0..self.data.len() {
            vis.push(self.visable(n));
        }

        vis
    }

    fn create_scenic_map(&self) -> Vec<usize> {
        let mut vis = Vec::with_capacity(self.data.len());

        for n in 0..self.data.len() {
            vis.push(self.tree_score(n));
        }

        vis
    }
}

#[cfg(test)]
mod tests {
    use super::{Map, Visable};

    const INPUT: &[u8] = b"30373
25512
65332
33549
35390";

    #[test]
    fn test_map_new() {
        let map = Map::new(INPUT);

        assert_eq!(map.x, 5);
        assert_eq!(map.y, 5);

        let mut ans = Visable::new();
        ans.Top();
        ans.Left();
        assert_eq!(map.visable(0), ans);

        let mut ans = Visable::new();
        ans.Top();
        ans.Right();
        assert_eq!(map.visable(4), ans);

        let mut ans = Visable::new();
        ans.Bottom();
        ans.Right();
        assert_eq!(map.visable(24), ans);

        let mut ans = Visable::new();
        ans.Bottom();
        ans.Right();
        ans.Left();
        ans.Top();
        assert_eq!(map.visable(23), ans);

        let mut ans = Visable::new();
        ans.Bottom();
        ans.Top();
        ans.Left();
        ans.Right();
        assert_eq!(map.visable(10), ans);
    }

    #[test]
    fn test_map_direction() {
        let map = Map::new(INPUT);

        let dir = map.create_direction_map();

        assert_eq!(dir.len(), 25);

        let total = dir.iter().fold(0, |acc, f| acc + usize::from(f.0 != 0));

        assert_eq!(total, 21);
    }

    #[test]
    fn test_map_scenic() {
        let map = Map::new(INPUT);

        let dir = map.create_scenic_map();

        assert_eq!(dir.len(), 25);

        assert_eq!(dir[7], 4);
        assert_eq!(dir[17], 8);
    }

    #[test]
    fn test_example() {}
}
