use std::fs;

fn main() {
    let input = fs::read_to_string("src/03/input.txt").expect("File not found");

    let sum = calc_priorities_sum(input.as_str());

    println!("The priority sum is {sum}.");
}

fn calc_priorities_sum(input: &str) -> u64 {
    let mut priorities = Vec::<u64>::new();

    let lines: Vec<&str> = input.lines().collect();
    for chunk in lines.chunks(3) {
        let (one, two, three) = (chunk[0], chunk[1], chunk[2]);
        for char in one.chars() {
            if two.contains(char) && three.contains(char) {
                priorities.push(get_priority(char));
                break;
            }
        }
    }

    priorities.iter().sum()
}

fn get_priority(char: char) -> u64 {
    let ascii = char as u64;
    if char.is_lowercase() {
        return ascii - ('a' as u64) + 1;
    } else if char.is_uppercase() {
        return ascii - ('A' as u64) + 27;
    }
    0
}

#[test]
fn test() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
                        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
                        PmmdzqPrVvPwwTWBwg\n\
                        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
                        ttgJtRGJQctTZtZT\n\
                        CrZsJsPPZsGzwwsLwLmpwMDw";

    assert_eq!(calc_priorities_sum(input), 70);
}
