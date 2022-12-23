use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/20.txt").unwrap();

    let coordinate_sum = mix(input.as_str());

    println!("The sum of the coordinates is {coordinate_sum}");
}

const DECRYPTION_KEY: i64 = 811589153;

fn mix(input: &str) -> i64 {
    let original = input
        .split("\n")
        .filter(|&s| !s.trim().is_empty())
        .map(|s| s.trim().parse::<i64>().unwrap() * DECRYPTION_KEY)
        .enumerate()
        .collect::<Vec<_>>();
    let mut mixed = original.clone();

    for _ in 0..10 {
        // 10 rounds of mixing
        for n in original.iter() {
            let old_index = mixed.iter().position(|x| x == n).unwrap();
            let removed = mixed.remove(old_index);

            let size = mixed.len() as i64;
            let new_index = (old_index as i64 + n.1).rem_euclid(size) as usize;

            mixed.insert(new_index, removed);
        }
    }

    let zero_index = mixed.iter().position(|(_, n)| *n == 0).unwrap();
    let (first, second, third) = (
        mixed[(zero_index + 1000) % mixed.len()].1,
        mixed[(zero_index + 2000) % mixed.len()].1,
        mixed[(zero_index + 3000) % mixed.len()].1,
    );

    first + second + third
}

#[test]
fn test() {
    let input = "1\n\
                2\n\
                -3\n\
                3\n\
                -2\n\
                0\n\
                4";

    mix(input);

    assert_eq!(mix(input), 1623178306);
}
