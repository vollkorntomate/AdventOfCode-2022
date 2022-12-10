extern crate core;

use std::collections::HashMap;
use std::fs;
use std::str::SplitWhitespace;

static TOTAL_DISK_SPACE: u64 = 70_000_000;
static MIN_UNUSED_SPACE: u64 = 30_000_000;

struct State {
    cwd: Vec<String>,
    files: HashMap<String, Vec<(String, u64)>>,
}

impl State {
    fn new() -> State {
        State {
            cwd: vec![],
            files: HashMap::new(),
        }
    }

    fn handle_line(&mut self, line: &str) {
        let mut split = line.split_whitespace();

        if line.starts_with("$") {
            split.next();
            self.handle_command(&mut split)
        } else {
            self.handle_output(&mut split)
        }
    }

    fn handle_command(&mut self, split: &mut SplitWhitespace) {
        match split.next() {
            Some("cd") => {
                match split.next() {
                    Some("..") => {
                        self.cwd.pop();
                    }
                    Some(dir) => {
                        if !self.files.contains_key(dir) {
                            self.files.insert(self.cwd_with_path(dir), vec![]);
                        }
                        self.cwd.push(String::from(dir));
                    }
                    _ => panic!("Unexpected end of match"),
                };
            }
            Some("ls") => {}
            _ => panic!("Unexpected end of match"),
        }
    }

    fn handle_output(&mut self, split: &mut SplitWhitespace) {
        match split.next() {
            Some("dir") => {
                let path = self.cwd_with_path(split.next().expect("Directory name expected"));
                self.files.insert(path, vec![]);
            }
            Some(file) => {
                let size: u64 = file.parse().expect("Not a size");
                let name = split.next().expect("File name expected").to_string();

                let dir = self
                    .files
                    .get_mut(self.cwd().as_str())
                    .expect("Existing dir expected");
                dir.push((name, size));
            }
            _ => panic!("Unexpected end of match"),
        }
    }

    fn cwd(&self) -> String {
        match self.cwd.len() {
            0 => String::new(),
            1 => String::from("/"),
            _ => String::from("/") + self.cwd[1..].join("/").as_str(),
        }
    }

    fn cwd_with_path(&self, path: &str) -> String {
        match self.cwd.len() {
            0 => String::from(path),
            1 => String::from("/") + path,
            _ => String::from("/") + self.cwd[1..].join("/").as_str() + "/" + path,
        }
    }

    fn sum_dir(&self, dir: &str) -> u64 {
        self.files
            .iter()
            .filter(|(name, _)| name.starts_with(dir))
            .flat_map(|(_, files)| files)
            .map(|(_, size)| size)
            .sum()
    }
}

fn main() {
    let input = fs::read_to_string("src/07/input.txt").expect("File not found");
    let input = input.as_str();
    let mut state = State::new();

    parse_lines(input, &mut state);

    let sum_under_100k = calc_size_sum_under_100k(&state);
    let size_to_free = find_smallest(&state);

    println!("Sum of all files under 100k is {}", sum_under_100k);
    println!(
        "The smallest directory to free up space has size {}",
        size_to_free
    );
}

fn parse_lines(input: &str, state: &mut State) {
    for line in input.lines() {
        state.handle_line(line);
    }
}

fn calc_size_sum_under_100k(state: &State) -> u64 {
    state
        .files
        .keys()
        .map(String::as_str)
        .map(|dir| state.sum_dir(dir))
        .filter(|size| size <= &100_000_u64)
        .sum()
}

fn find_smallest(state: &State) -> u64 {
    let currently_used = state.sum_dir("/");
    let currently_unused = TOTAL_DISK_SPACE - currently_used;
    let needed = MIN_UNUSED_SPACE - currently_unused;

    state
        .files
        .keys()
        .map(String::as_str)
        .map(|dir| state.sum_dir(dir))
        .filter(|&size| size >= needed)
        .min()
        .expect("No minimum found!")
}

#[test]
fn test() {
    let input = "$ cd /\n\
                    $ ls\n\
                    dir a\n\
                    14848514 b.txt\n\
                    8504156 c.dat\n\
                    dir d\n\
                    $ cd a\n\
                    $ ls\n\
                    dir e\n\
                    29116 f\n\
                    2557 g\n\
                    62596 h.lst\n\
                    $ cd e\n\
                    $ ls\n\
                    584 i\n\
                    $ cd ..\n\
                    $ cd ..\n\
                    $ cd d\n\
                    $ ls\n\
                    4060174 j\n\
                    8033020 d.log\n\
                    5626152 d.ext\n\
                    7214296 k";

    let mut state = State::new();
    parse_lines(input, &mut state);

    assert_eq!(calc_size_sum_under_100k(&state), 95437);
}
