use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/25.txt").unwrap();

    let sum = snafu_sum(input.as_str());

    println!("The sum in SNAFU is {sum}");
}

fn snafu_sum(input: &str) -> String {
    let dec_sum = input.lines().map(snafu_to_dec).sum();

    dec_to_snafu(dec_sum)
}

fn snafu_to_dec(snafu: &str) -> i64 {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| 5_i64.pow(i as u32) * char_to_digit(c))
        .sum()
}

fn dec_to_snafu(dec: i64) -> String {
    let mut dec = dec;
    let mut snafu = String::new();

    while dec != 0 {
        let rem = dec % 5;

        let digit_5 = match rem {
            0..=2 => rem,
            3 => -2,
            4 => -1,
            _ => unreachable!(),
        };
        snafu.push(digit_to_char(digit_5));

        dec -= digit_5;
        dec /= 5;
    }

    snafu.chars().rev().collect::<String>()
}

fn char_to_digit(char: char) -> i64 {
    match char {
        '0'..='2' => char.to_digit(10).unwrap() as i64,
        '-' => -1,
        '=' => -2,
        _ => unreachable!(),
    }
}

fn digit_to_char(digit: i64) -> char {
    match digit {
        0..=2 => char::from_digit(digit as u32, 10).unwrap(),
        -1 => '-',
        -2 => '=',
        _ => unreachable!(),
    }
}

#[test]
fn test() {
    let input = "1=-0-2\n\
                12111\n\
                2=0=\n\
                21\n\
                2=01\n\
                111\n\
                20012\n\
                112\n\
                1=-1=\n\
                1-12\n\
                12\n\
                1=\n\
                122";

    assert_eq!(snafu_sum(input), "2=-1=0")
}
