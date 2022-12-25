use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("inputs/23.txt").unwrap();

    let empty_fields = empty_fields(input.as_str());

    println!("There are {empty_fields} empty fields in the rectangle.");
}

type Coord = (i32, i32);
type Field = HashSet<Coord>;
const DIRECTIONS: [Coord; 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)]; // N,S,W,E

fn empty_fields(input: &str) -> u64 {
    let mut field = parse_input(input);

    for round in 0..10 {
        let propositions = simulate_round(&mut field, round % 4);

        for (&new, old) in propositions.iter() {
            field.remove(old);
            field.insert(new);
        }
    }

    field_size(&field) - (field.len() as u64)
}

fn simulate_round(field: &mut Field, direction: usize) -> HashMap<Coord, Coord> {
    let mut propositions = HashMap::<Coord, Coord>::new(); // new -> old
    let mut duplicate_propositions = Field::new();
    for &(x, y) in field.iter() {
        if !is_elf_adjacent(field, (x, y)) {
            continue;
        }

        let mut proposal: Option<Coord> = None;
        for i in 0..4 {
            let (dx, dy) = DIRECTIONS[(direction + i) % 4];
            if dx == 0 && !is_elf_adjacent_x(field, (x, y + dy)) {
                // N or S
                proposal = Some((x, y + dy));
                break;
            } else if dy == 0 && !is_elf_adjacent_y(field, (x + dx, y)) {
                // W or E
                proposal = Some((x + dx, y));
                break;
            }
        }

        if let Some(proposition) = proposal {
            if propositions.contains_key(&proposition) {
                propositions.remove(&proposition);
                duplicate_propositions.insert(proposition);
            } else if !duplicate_propositions.contains(&proposition) {
                propositions.insert(proposition, (x, y));
            }
        }
    }

    propositions
}

fn is_elf_adjacent(field: &Field, (x, y): Coord) -> bool {
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if field.contains(&(x + dx, y + dy)) {
                return true;
            }
        }
    }
    false
}

fn is_elf_adjacent_x(field: &Field, (x, y): Coord) -> bool {
    field.iter().any(|&(ix, iy)| iy == y && (ix - x).abs() <= 1)
}

fn is_elf_adjacent_y(field: &Field, (x, y): Coord) -> bool {
    field.iter().any(|&(ix, iy)| ix == x && (iy - y).abs() <= 1)
}

fn parse_input(input: &str) -> Field {
    let mut field = Field::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                field.insert((x as i32, y as i32));
            }
        }
    }

    field
}

fn field_size(field: &Field) -> u64 {
    let ((min_x, min_y), (max_x, max_y)) = min_max(field);

    ((max_x - min_x + 1) * (max_y - min_y + 1)) as u64
}

fn min_max(field: &Field) -> (Coord, Coord) {
    let min_x = field.iter().map(|&(x, _)| x).min().unwrap();
    let max_x = field.iter().map(|&(x, _)| x).max().unwrap();
    let min_y = field.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = field.iter().map(|&(_, y)| y).max().unwrap();

    ((min_x, min_y), (max_x, max_y))
}

// fn print_field(field: &Field) {
//     let ((min_x, min_y), (max_x, max_y)) = min_max(field);

//     for y in min_y..=max_y {
//         for x in min_x..=max_x {
//             if field.contains(&(x,y)) {
//                 print!("#")
//             } else {
//                 print!(".")
//             }
//         }
//         println!("");
//     }
// }

#[test]
fn test() {
    let input = "....#..\n\
                    ..###.#\n\
                    #...#.#\n\
                    .#...##\n\
                    #.###..\n\
                    ##.#.##\n\
                    .#..#..";

    assert_eq!(empty_fields(input), 110);
}
