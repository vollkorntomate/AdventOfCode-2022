use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/01.txt").expect("File not found");

    let mut calories: Vec<u64> = vec![];

    let mut sum = 0u64;
    for line in input.lines() {
        if line.is_empty() {
            calories.push(sum);
            sum = 0;
            continue;
        }
        sum += line.parse::<u64>().expect("Not a number!");
    }

    calories.sort();
    calories.reverse();

    let top = (
        calories.get(0).unwrap(),
        calories.get(1).unwrap(),
        calories.get(2).unwrap(),
    );
    let top_sum = top.0 + top.1 + top.2;
    println!(
        "The top three have {}, {} and {} respectively, sum: {}",
        top.0, top.1, top.2, top_sum
    );
}
