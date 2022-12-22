use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("src/22/input.txt").unwrap();

    let password = find_password(input.as_str());

    println!("The final password is {password}");
}

type Coord = (i32, i32);
type Board = HashMap<Coord, bool>;
const FACING: [Coord; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)]; // R,D,L,U

fn find_password(input: &str) -> i32 {
    let (board, instructions) = parse_input(input);

    let (mut x, mut y) = find_start(&board);
    let mut facing = 0;

    for instruction in instructions {
        match instruction {
            Instruction::TurnL => facing = (facing - 1_i32).rem_euclid(4),
            Instruction::TurnR => facing = (facing + 1) % 4,
            Instruction::Move(steps) => {
                let (dx, dy) = FACING[facing as usize];
                let ((min_x, max_x), (min_y, max_y)) = min_max(&board, (x, y));
                for _ in 0..steps {
                    let (next_x, next_y) =
                        (wrap(x + dx, (min_x, max_x)), wrap(y + dy, (min_y, max_y)));
                    if board[&(next_x, next_y)] {
                        break;
                    }

                    x = next_x;
                    y = next_y;
                }
            }
        }
    }

    1000 * (y + 1) + 4 * (x + 1) + facing
}

fn wrap(val: i32, (min, max): Coord) -> i32 {
    (val - min).rem_euclid(max - min + 1) + min
}

fn min_max(board: &Board, (cx, cy): Coord) -> (Coord, Coord) {
    let min_x = board
        .keys()
        .filter(|(_, y)| *y == cy)
        .map(|(x, _)| *x)
        .min()
        .unwrap();
    let max_x = board
        .keys()
        .filter(|(_, y)| *y == cy)
        .map(|(x, _)| *x)
        .max()
        .unwrap();
    let min_y = board
        .keys()
        .filter(|(x, _)| *x == cx)
        .map(|(_, y)| *y)
        .min()
        .unwrap();
    let max_y = board
        .keys()
        .filter(|(x, _)| *x == cx)
        .map(|(_, y)| *y)
        .max()
        .unwrap();

    ((min_x, max_x), (min_y, max_y))
}

fn find_start(board: &Board) -> Coord {
    let ((min_x, _), _) = min_max(board, (0, 0));

    (min_x, 0)
}

fn parse_input(input: &str) -> (Board, Vec<Instruction>) {
    let mut board = Board::new();

    let mut input_parts = input.split("\n\n");
    let board_input = input_parts.next().unwrap();
    let instructions_input = input_parts.next().unwrap();

    for (y, line) in board_input.lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            match tile {
                ' ' => continue,
                '#' => board.insert((x as i32, y as i32), true),
                '.' => board.insert((x as i32, y as i32), false),
                _ => panic!("Unexpected input {tile}"),
            };
        }
    }

    (board, parse_instructions(instructions_input.trim()))
}

fn parse_instructions(instructions_input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::<Instruction>::new();

    let mut digits = String::from("");
    for c in instructions_input.chars() {
        if c.is_numeric() {
            digits.push(c);
        } else {
            instructions.push(Instruction::Move(digits.parse().unwrap()));
            digits.clear();
            match c {
                'R' => instructions.push(Instruction::TurnR),
                'L' => instructions.push(Instruction::TurnL),
                _ => unreachable!(),
            }
        }
    }
    instructions.push(Instruction::Move(digits.parse().unwrap()));

    instructions
}

#[derive(Debug)]
enum Instruction {
    Move(i32),
    TurnL,
    TurnR,
}

#[test]
fn test() {
    let input = fs::read_to_string("src/22/test.txt").unwrap();

    assert_eq!(find_password(input.as_str()), 6032);
}
