#![deny(clippy::pedantic)]

use core::str::from_utf8;

type Ipos = i32;

type Upos = u32;

const Y_ROW: Ipos = 2_000_000;

#[derive(Debug, PartialEq, Eq)]

struct Point(Ipos, Ipos);

impl Point {
    // input = "x=123, y=123"

    fn parse(input: &[u8]) -> Self {
        let pos: Vec<Ipos> = input
            .split(|v| *v == b',')
            .map(|v| v.split(|v| *v == b'=').nth(1).unwrap())
            .map(|v| from_utf8(v).unwrap().parse::<Ipos>().unwrap())
            .collect();

        Self(pos[0], pos[1])
    }

    fn distance(&self, beacon: &Point) -> Upos {
        self.0.abs_diff(beacon.0) + self.1.abs_diff(beacon.1)
    }

    fn in_range(&self, dis: Upos, y: Ipos) -> Option<Upos> {
        let y_dis = self.1.abs_diff(y);

        if y_dis > dis {
            None
        } else {
            Some(dis - y_dis)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]

struct Info {
    sensor: Point,
    beacon: Point,
    distance: Upos,
}

fn decode(input: &[u8]) -> Vec<Info> {
    let mut ret = Vec::with_capacity(20);

    for line in input.split(|v| *v == b'\n') {
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let mut items = line.split(|v| *v == b':');
        let sensor = Point::parse(items.next().unwrap());
        let beacon = Point::parse(items.next().unwrap());
        let distance = sensor.distance(&beacon);

        ret.push(Info {
            sensor,
            beacon,
            distance,
        });
    }

    ret
}

fn main() {
    let input = include_bytes!("../input/input.txt");

    let data = decode(input);

    let mut beacon_not_at = Vec::<Ipos>::with_capacity(1000);

    for point in data {
        let sensor = point.sensor;

        let dis = point.distance;

        if let Some(r) = sensor.in_range(dis, Y_ROW) {
            let r = Ipos::try_from(r).unwrap();

            for i in sensor.0 - r..sensor.0 + r {
                beacon_not_at.push(i);
            }
        }

        if point.beacon.1 == Y_ROW {
            beacon_not_at.push(point.beacon.0);
        }

        beacon_not_at.sort_unstable();

        beacon_not_at.dedup();
    }

    println!("y={Y_ROW}: num: {}", beacon_not_at.len());
}

#[cfg(test)]

mod tests {

    use super::{decode, Info, Ipos, Point};

    const INPUT: &[u8] = b"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_point() {
        assert_eq!(Point::parse(b"x=9, y=16"), Point(9, 16));

        assert_eq!(
            Point::parse(b"x=-2413129, y=-12312216"),
            Point(-2413129, -12312216)
        );
    }

    #[test]
    fn test_decode() {
        let mut line = INPUT.split(|v| *v == b'\n');

        assert_eq!(
            decode(line.next().unwrap()),
            vec![Info {
                sensor: Point(2, 18),
                beacon: Point(-2, 15),
                distance: 7
            }]
        );

        assert_eq!(
            decode(line.next().unwrap()),
            vec![Info {
                sensor: Point(9, 16),
                beacon: Point(10, 16),
                distance: 1
            }]
        );
    }

    #[test]

    fn test_range() {
        let point = Point(0, 0);

        assert_eq!(point.in_range(5, 5), Some(0));
        assert_eq!(point.in_range(5, 0), Some(5));
        assert_eq!(point.in_range(5, -5), Some(0));
        assert_eq!(point.in_range(5, -1), Some(4));
        assert_eq!(point.in_range(5, 6), None);
        let point = Point(1232130, 1231230);
        assert_eq!(point.in_range(5, 6), None);
    }

    #[test]

    fn test_example() {
        let data = decode(INPUT);

        let mut convert = Vec::<Ipos>::with_capacity(30);

        for point in data {
            let sensor = point.sensor;
            let dis = point.distance;

            if let Some(r) = sensor.in_range(dis, 10) {
                let r = Ipos::try_from(r).unwrap();

                for i in sensor.0 - r..sensor.0 + r {
                    convert.push(i);
                }
            }

            convert.sort();
            convert.dedup();
        }

        assert_eq!(convert.len(), 26);
    }
}
