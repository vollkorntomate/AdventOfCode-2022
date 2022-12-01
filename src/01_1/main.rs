use std::{fs};

fn main() {
    let input = fs::read_to_string("src/01_1/input.txt").expect("File not found");
    
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

    let mut max = (0usize, 0u64);
    for cal in calories.iter().enumerate() {
        if cal.1 > &max.1 {
            max = (cal.0, *cal.1);
        }
    }

    println!("Max value is {} for elf n° {}.", max.1, max.0);
}
