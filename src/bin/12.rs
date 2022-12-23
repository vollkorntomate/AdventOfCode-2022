use std::{
    collections::{HashSet, VecDeque},
    fs,
};

type Grid = Vec<Vec<u8>>;
type Position = (usize, usize);

fn main() {
    let input = fs::read_to_string("inputs/12.txt").unwrap();

    let steps = find_start_point(input.as_str());

    println!("To reach the goal, {} steps are required", steps);
}

fn find_start_point(input: &str) -> u64 {
    let (grid, start, end) = parse_input(input);

    let mut min: u64 = number_of_steps(&grid, start, end).unwrap();

    for y in 0..grid.len() {
        for x in 0..grid.first().unwrap().len() {
            if let Some(0) = grid.get(y).and_then(|row| row.get(x)) {
                if let Some(distance) = number_of_steps(&grid, (x, y), end) {
                    min = distance.min(min);
                }
            }
        }
    }

    min
}

fn number_of_steps(grid: &Grid, start: Position, end: Position) -> Option<u64> {
    let mut visited = HashSet::<Position>::new();
    let mut candidates = VecDeque::<(Position, u64)>::new();
    candidates.push_back((start, 0));
    while let Some(((x, y), len)) = candidates.pop_front() {
        if (x, y) == end {
            return Some(len);
        }
        let height = grid[y][x];

        for (diff_x, diff_y) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let (new_x, new_y) = (
                (x as isize + diff_x) as usize,
                (y as isize + diff_y) as usize,
            );

            if let Some(&next) = grid.get(new_y).and_then(|row| row.get(new_x)) {
                if !visited.contains(&(new_x, new_y)) && next <= height + 1 {
                    visited.insert((new_x, new_y));
                    candidates.push_back(((new_x, new_y), len + 1));
                }
            }
        }
    }
    None
}

fn parse_input(input: &str) -> (Grid, Position, Position) {
    let mut start: Position = (0, 0);
    let mut end: Position = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    'S' => {
                        start = (x, y);
                        0
                    }
                    'E' => {
                        end = (x, y);
                        25
                    }
                    c => (c as u8) - ('a' as u8),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (grid, start, end)
}

#[test]
fn test() {
    let input = "Sabqponm\n\
                        abcryxxl\n\
                        accszExk\n\
                        acctuvwj\n\
                        abdefghi";

    assert_eq!(find_start_point(input), 29);
}
