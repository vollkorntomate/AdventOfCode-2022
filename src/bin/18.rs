use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("inputs/18.txt").unwrap();

    let sides = count_cube_sides(input.as_str());

    println!("There are {sides} sides that aren't connected");
}

type Coord = (i32, i32, i32);
type Grid = HashSet<Coord>;
const DIFF_COORDS: [Coord; 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

fn count_cube_sides(input: &str) -> u64 {
    let grid = parse_input(input);

    let mut count = 0;
    for (coord_x, coord_y, coord_z) in grid.iter() {
        for (dx, dy, dz) in DIFF_COORDS {
            if !grid.contains(&(coord_x + dx, coord_y + dy, coord_z + dz)) {
                count += 1
            }
        }
    }

    count
}

fn parse_input(input: &str) -> Grid {
    let mut grid = Grid::new();

    for line in input.lines() {
        let mut split = line.split(",");
        let coord: Coord = (
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        );

        grid.insert(coord);
    }

    grid
}

#[test]
fn test() {
    let input = "2,2,2\n\
    1,2,2\n\
    3,2,2\n\
    2,1,2\n\
    2,3,2\n\
    2,2,1\n\
    2,2,3\n\
    2,2,4\n\
    2,2,6\n\
    1,2,5\n\
    3,2,5\n\
    2,1,5\n\
    2,3,5";

    assert_eq!(count_cube_sides(input), 64);
}
