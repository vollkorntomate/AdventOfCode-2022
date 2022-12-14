use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("src/14/input.txt").unwrap();

    let resting_sand = count_resting_sand(input.as_str());

    println!("The number of resting sand units is {}", resting_sand);
}

type Pos = (i16, i16);
type Grid = HashMap<Pos, State>;

fn count_resting_sand(input: &str) -> u64 {
    let mut grid = parse_input(input);

    print_grid(&grid);

    let mut sand: Pos = (500, 0);
    while let None = grid.get(&sand) {
        let mut make_solid = true;
        for (x, y) in [(0, 1), (-1, 1), (1, 1)] {
            let next = (sand.0 + x, sand.1 + y);
            if let None = grid.get(&next) {
                sand = next;
                make_solid = false;
                break;
            }
        }
        if make_solid {
            grid.insert(sand, State::RestingSand);
            sand = (500, 0);
        }
    }

    print_grid(&grid);

    grid.values().filter(|v| **v == State::RestingSand).count() as u64
}

fn parse_input(input: &str) -> Grid {
    let mut grid = Grid::new();

    for line in input.lines() {
        let mut split = line.split(" -> ");

        let mut start = parse_coord(split.next().unwrap());
        while let Some(pos_str) = split.next() {
            let coord = parse_coord(pos_str);
            for x in start.0.min(coord.0)..=start.0.max(coord.0) {
                grid.insert((x, start.1), State::Rock);
            }
            for y in start.1.min(coord.1)..=start.1.max(coord.1) {
                grid.insert((start.0, y), State::Rock);
            }

            start = coord;
        }
    }

    let ((min_x, _), (max_x, max_y)) = min_max(&grid);
    for x in (min_x - 1000)..=(max_x + 1000) {
        // assuming 1000 is large enough
        grid.insert((x, max_y + 2), State::Rock);
    }

    grid
}

fn parse_coord(coord: &str) -> Pos {
    let mut split = coord.split(",");

    (
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap().parse().unwrap(),
    )
}

/// Returns ((min_x, min_y), (max_x, max_y))
fn min_max(grid: &Grid) -> (Pos, Pos) {
    let min_x = grid.keys().min_by_key(|(x, _)| x).unwrap().0;
    let max_x = grid.keys().max_by_key(|(x, _)| x).unwrap().0;
    let min_y = grid.keys().min_by_key(|(_, y)| y).unwrap().1;
    let max_y = grid.keys().max_by_key(|(_, y)| y).unwrap().1;

    ((min_x, min_y), (max_x, max_y))
}

fn print_grid(grid: &Grid) {
    let ((mut min_x, min_y), (mut max_x, max_y)) = min_max(grid);
    min_x += 995;
    max_x -= 995;
    println!("x: {min_x} - {max_x}");
    for y in min_y..=max_y {
        print!("{y} ");
        for x in min_x..=max_x {
            let char = match grid.get(&(x, y)) {
                None => '.',
                Some(State::Rock) => '#',
                Some(State::RestingSand) => 'o',
            };
            print!("{}", char);
        }
        print!("\n");
    }
}

#[derive(PartialEq)]
enum State {
    Rock,
    RestingSand,
}

#[test]
fn test() {
    let input = "498,4 -> 498,6 -> 496,6\n\
                        503,4 -> 502,4 -> 502,9 -> 494,9";

    assert_eq!(count_resting_sand(input), 93);
}
