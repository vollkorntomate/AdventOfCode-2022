use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("src/15/input.txt").unwrap();

    let tuning_frequency = find_tuning_frequency(input.as_str(), 4_000_000);

    println!("The tuning frequency is {}.", tuning_frequency);
}

type Pos = (i64, i64);
type Grid = HashMap<Pos, i64>; // pos -> distance

fn find_tuning_frequency(input: &str, limit: i64) -> i64 {
    let (sensors, _) = parse_input(input);

    for (&(x, y), &distance) in &sensors {
        for (dir_x, dir_y) in [(1, 1), (-1, 1), (-1, -1), (1, -1)] {
            for d in 0..distance {
                let try_x = x + dir_x * d;
                let remaining = distance - d;
                let try_y = y + dir_y * (remaining + 1); // one further than reach
                if try_x < 0 || try_y < 0 || try_x > limit || try_y > limit {
                    continue;
                }
                if sensors
                    .iter()
                    .all(|(&pos, &sd)| manhattan_distance(pos, (try_x, try_y)) > sd)
                {
                    return try_x * 4_000_000 + try_y;
                }
            }
        }
    }
    0
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

    assert_eq!(find_tuning_frequency(input, 20), 56_000_011);
}
