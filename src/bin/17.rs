use std::{fs, usize};

fn main() {
    let input = fs::read_to_string("inputs/17.txt").unwrap();

    let height = tower_height(input.as_str());

    println!("The tower is {height} rocks tall.");
}

const CHAMBER_WIDTH: usize = 7;
type Chamber = Vec<[bool; CHAMBER_WIDTH]>;

type Rock = Vec<Vec<bool>>;

fn tower_height(input: &str) -> u64 {
    let mut chamber = Chamber::new();
    chamber.push([true; 7]); // floor

    let rocks = [
        vec![vec![true, true, true, true]],
        vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ],
        vec![
            vec![true, true, true],
            vec![false, false, true],
            vec![false, false, true],
        ],
        vec![vec![true], vec![true], vec![true], vec![true]],
        vec![vec![true, true], vec![true, true]],
    ];

    let mut moves = input.trim().chars().peekable();
    for rock_i in 0..2022 {
        let (mut rock_x, mut rock_y) = (2, chamber.len() as i64 + 3); // bottom-left corner
        let mut is_solid = false;
        while !is_solid {
            if let None = moves.peek() {
                moves = input.trim().chars().peekable(); // reset iterator
            }
            let horizontal: i64 = match moves.next() {
                Some('<') => -1,
                Some('>') => 1,
                _ => panic!("Unexpected input"),
            };

            let rock = rocks.get(rock_i % 5).unwrap();
            if does_fit_horizontally(rock, &chamber, (rock_x, rock_y), horizontal) {
                rock_x += horizontal;
            }

            if does_fit_vertically(rock, &chamber, (rock_x, rock_y)) {
                rock_y -= 1;
            } else {
                is_solid = true;
                make_solid(&mut chamber, rock, (rock_x, rock_y));
            }
        }
    }

    chamber.len() as u64 - 1
}

fn does_fit_horizontally(
    rock: &Rock,
    chamber: &Chamber,
    (rock_x, rock_y): (i64, i64),
    horizontal: i64,
) -> bool {
    let new_x = rock_x + horizontal;
    let height = rock.len();
    let width = rock[0].len();
    if new_x < 0 || new_x as usize + width - 1 >= CHAMBER_WIDTH {
        return false;
    }

    let new_x = new_x as usize;
    for ry in 0..height {
        if let Some(row) = chamber.get(rock_y as usize + ry) {
            for rx in 0..width {
                if row[new_x + rx] && rock[ry][rx] {
                    return false;
                }
            }
        }
    }
    true
}

fn does_fit_vertically(rock: &Rock, chamber: &Chamber, (rock_x, rock_y): (i64, i64)) -> bool {
    let new_y = rock_y - 1;
    if new_y < 0 {
        return false;
    }
    let new_y = new_y as usize;
    let height = rock.len();
    let width = rock[0].len();

    for ry in 0..height {
        if let Some(row) = chamber.get(new_y + ry) {
            for rx in 0..width {
                if row[rock_x as usize + rx] && rock[ry][rx] {
                    return false;
                }
            }
        }
    }
    true
}

/* fn print_chamber(chamber: &Chamber) {
    for row in chamber.iter().skip(1).rev() {
        print!("|");
        for &solid in row {
            print!("{}", if solid { '#' } else { '.' })
        }
        println!("|");
    }
    println!("+-------+");
} */

fn make_solid(chamber: &mut Chamber, rock: &Rock, (rock_x, rock_y): (i64, i64)) {
    let height = rock.len();
    let width = rock[0].len();
    if rock_y as usize + height > chamber.len() {
        let missing = rock_y as usize + height - chamber.len();
        for _ in 0..missing {
            chamber.push([false; CHAMBER_WIDTH]);
        }
    }
    for y in 0..height {
        for x in 0..width {
            chamber[y + rock_y as usize][x + rock_x as usize] |= rock[y][x];
        }
    }
}

#[test]
fn test() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    assert_eq!(tower_height(input), 3068);
}
