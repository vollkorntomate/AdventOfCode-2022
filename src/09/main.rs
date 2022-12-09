use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("src/09/input.txt").unwrap();

    let visits = count_tail_visits(input.as_str());

    println!("The tail visits {visits} positions.");
}

fn count_tail_visits(input: &str) -> usize {
    let mut knots = [(0i64, 0i64); 10];
    let mut tail_positions = HashSet::<(i64, i64)>::new();
    tail_positions.insert((0, 0));
    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let direction = *split.get(0).unwrap();
        let count: u64 = split.get(1).unwrap().parse().unwrap();
        let move_dir = move_direction(direction);

        for _ in 0..count {
            knots[0] = (knots[0].0 + move_dir.0, knots[0].1 + move_dir.1); // first knot moves
            for i in 1..knots.len() {
                let head = knots[i - 1];
                let mut tail = knots[i];

                let distance = distance(&head, &tail);
                if are_separated(&distance) {
                    let tail_move = tail_move(&distance);
                    tail = (tail.0 + tail_move.0, tail.1 + tail_move.1);

                    if i == knots.len() - 1 {
                        // last knot
                        tail_positions.insert(tail);
                    }
                }
                knots[i] = tail;
            }
        }
    }
    tail_positions.len()
}

#[inline]
fn distance(head: &(i64, i64), tail: &(i64, i64)) -> (i64, i64) {
    (tail.0 - head.0, tail.1 - head.1)
}

#[inline]
fn are_separated(distance: &(i64, i64)) -> bool {
    distance.0.abs() > 1 || distance.1.abs() > 1
}

fn tail_move(distance: &(i64, i64)) -> (i64, i64) {
    let mut x = distance.0;
    let mut y = distance.1;
    if distance.0.abs() > 1 {
        x = distance.0.signum() * 1;
    }
    if distance.1.abs() > 1 {
        y = distance.1.signum() * 1;
    }

    (-x, -y)
}

fn move_direction(direction: &str) -> (i64, i64) {
    match direction {
        "D" => (0, -1),
        "U" => (0, 1),
        "L" => (-1, 0),
        "R" => (1, 0),
        _ => panic!("Unexpected value"),
    }
}

#[test]
fn test() {
    let input = "R 5\n\
                        U 8\n\
                        L 8\n\
                        D 3\n\
                        R 17\n\
                        D 10\n\
                        L 25\n\
                        U 20";

    assert_eq!(count_tail_visits(input), 36);
}
