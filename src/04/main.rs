use std::fs;
use std::ops::RangeInclusive;

fn main() {
    let file = fs::read_to_string("src/04/input.txt").expect("File not found");

    let contained = count_contains(file.as_str());
    let overlapping = count_overlaps(file.as_str());

    println!("There are {} sections contained in one another.", contained);
    println!("There are {} sections overlapping one another.", overlapping);
}

/// Part 1
fn count_contains(input: &str) -> u64 {
    let predicate = |range1: RangeInclusive<u64>, range2: RangeInclusive<u64>| {
        (range2.start() <= range1.start() && range2.end() >= range1.end())
            || (range1.start() <= range2.start() && range1.end() >= range2.end())
    };

    count_predicate(input, predicate)
}

/// Part 2
fn count_overlaps(input: &str) -> u64 {
    let predicate = |range1: RangeInclusive<u64>, range2: RangeInclusive<u64>| {
        range1.start() <= range2.end() && range2.start() <= range1.end()
    };

    count_predicate(input, predicate)
}

fn count_predicate(
    input: &str,
    predicate: fn(RangeInclusive<u64>, RangeInclusive<u64>) -> bool,
) -> u64 {
    let mut count = 0u64;
    for line in input.lines() {
        let pair = parse_pairs(line);
        let range1 = parse_range(pair.0);
        let range2 = parse_range(pair.1);

        if predicate(range1, range2) {
            count += 1;
        }
    }
    count
}

fn parse_pairs(line: &str) -> (&str, &str) {
    let split: Vec<&str> = line.split(",").collect();

    (split.get(0).unwrap(), split.get(1).unwrap())
}

fn parse_range(pair: &str) -> RangeInclusive<u64> {
    let split: Vec<&str> = pair.split("-").collect();
    let pair: (u64, u64) = (
        split.get(0).unwrap().parse().unwrap(),
        split.get(1).unwrap().parse().unwrap(),
    );

    (pair.0)..=(pair.1)
}

#[test]
fn test() {
    let input = "2-4,6-8\n\
                        2-3,4-5\n\
                        5-7,7-9\n\
                        2-8,3-7\n\
                        6-6,4-6\n\
                        2-6,4-8";

    assert_eq!(count_contains(input), 2);
    assert_eq!(count_overlaps(input), 4);
}
