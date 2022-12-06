use std::collections::{HashSet, VecDeque};
use std::fs;

const N_DISTINCT_CHARACTERS: usize = 14; // Part 1: 4; Part 2: 14

fn main() {
    let input = fs::read_to_string("src/06/input.txt").expect("File not found");

    let marker_pos = find_marker(input.as_str());

    println!("Marker is at {}", marker_pos);
}

fn find_marker(input: &str) -> u64 {
    let mut buf = VecDeque::<char>::new();
    let mut pos = 0usize;

    for (i, char) in input.chars().enumerate() {
        buf.push_back(char);
        if i >= N_DISTINCT_CHARACTERS {
            buf.pop_front();

            if are_all_different(&buf) {
                pos = i + 1;
                break;
            }
        }
    }

    u64::try_from(pos).unwrap()
}

fn are_all_different(queue: &VecDeque<char>) -> bool {
    let set: HashSet<char> = queue.iter().map(char::clone).collect();

    queue.len() == set.len()
}

#[test]
fn test() {
    assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}
