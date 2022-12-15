use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("src/15/input.txt").unwrap();

    let count = count_no_positions(input.as_str(), 2_000_000);

    println!("There are {} positions that are not covered.", count);
}

type Pos = (i64, i64);
type Grid = HashMap<Pos, i64>; // pos -> distance

fn count_no_positions(input: &str, row: i64) -> u64 {
    let (sensors, beacons) = parse_input(input);

    let mut covered = HashSet::<i64>::new();
    for ((x, y), distance) in sensors {
        if row >= y - distance && row <= y + distance {
            let remaining = distance - (row - y).abs();
            for dx in (x - remaining)..=(x + remaining) {
                covered.insert(dx);
            }
        }
    }

    let beacon_count = beacons.iter().filter(|(_, y)| y == &row).count();

    (covered.len() - beacon_count) as u64
}

fn parse_input(input: &str) -> (Grid, HashSet<Pos>) {
    let mut sensors = Grid::new();
    let mut beacons = HashSet::<Pos>::new();
    for line in input.lines() {
        let mut split = line.split(":"); // [Sensor at x=2, y=18] [ closest beacon is at x=-2, y=15]
        let mut sensor_str = split.next().unwrap().split(",");
        let mut beacon_str = split.next().unwrap().split(",");

        let sensor: Pos = (
            sensor_str.next().unwrap()["Sensor at x=".len()..]
                .parse()
                .unwrap(),
            sensor_str.next().unwrap().trim()["y=".len()..]
                .parse()
                .unwrap(),
        );
        let beacon: Pos = (
            beacon_str.next().unwrap().trim()["closest beacon is at x=".len()..]
                .parse()
                .unwrap(),
            beacon_str.next().unwrap().trim()["y=".len()..]
                .parse()
                .unwrap(),
        );

        let distance = manhattan_distance(sensor, beacon);
        sensors.insert(sensor, distance);
        beacons.insert(beacon);
    }

    (sensors, beacons)
}

fn manhattan_distance(a: Pos, b: Pos) -> i64 {
    let (x, y) = ((b.0 - a.0).abs(), (b.1 - a.1).abs());

    x + y
}

#[test]
fn test() {
    let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
                Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
                Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
                Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
                Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
                Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
                Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
                Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
                Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
                Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
                Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
                Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
                Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
                Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    assert_eq!(count_no_positions(input, 10), 26);
}
