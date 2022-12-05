use std::error::Error;
use std::fs;
use std::str::Lines;

type Stacks = Vec<Vec<char>>;

fn main() {
    let input = fs::read_to_string("src/05/input.txt").expect("File not found");
    let mut input_iter = input.lines().into_iter();

    let mut stacks = parse_stack(&mut input_iter).unwrap();

    make_moves(&mut input_iter, &mut stacks);

    print_top(&stacks);
}

fn parse_stack(input: &mut Lines) -> Option<Stacks> {
    let mut lines = Vec::<String>::new();
    for line in input {
        if line.trim().is_empty() {
            break;
        }

        lines.push(line.to_string());
    }

    let n = lines
        .pop()?
        .split_whitespace()
        .collect::<Vec<&str>>()
        .last()?
        .parse::<u64>()
        .expect("Not a number");

    lines = lines.into_iter().map(|l| strip_line(l.as_str())).collect();
    lines.reverse();

    let mut stacks = init_stack(n);
    for line in lines {
        parse_stack_line(&line, &mut stacks);
    }

    Some(stacks)
}

fn make_moves(input: &mut Lines, stacks: &mut Stacks) {
    for line in input {
        let (n, from, to) = parse_move_line(line);

        for _ in 0..n {
            let val = stacks.get_mut(from - 1).unwrap().pop();
            if let Some(val) = val {
                stacks.get_mut(to - 1).unwrap().push(val);
            }
        }
    }
}

fn parse_move_line(line: &str) -> (u64, usize, usize) {
    let split = line.split(" ").collect::<Vec<&str>>();
    (
        split.get(1).unwrap().parse().unwrap(),
        split.get(3).unwrap().parse().unwrap(),
        split.get(5).unwrap().parse().unwrap(),
    )
}

fn parse_stack_line(line: &String, stacks: &mut Stacks) {
    for (i, char) in line.chars().enumerate() {
        match char {
            'A'..='Z' => stacks.get_mut(i).unwrap().push(char),
            _ => continue,
        }
    }
}

fn init_stack(n: u64) -> Stacks {
    let mut stack = Stacks::new();
    for _ in 0..n {
        stack.push(Vec::<char>::new());
    }

    stack
}

fn strip_line(line: &str) -> String {
    String::from(line)
        .replace("    ", "-")
        .replace(" [", "")
        .replace("[", "")
        .replace("]", "")
}

fn print_top(stacks: &Stacks) {
    let mut out = String::from("");
    for stack in stacks {
        let val = stack.last();
        if let Some(val) = val {
            out += &val.to_string();
        }
    }
    println!("Output: {}", out);
}
