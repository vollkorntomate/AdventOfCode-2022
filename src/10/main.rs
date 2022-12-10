use std::fs;

fn main() {
    let input = fs::read_to_string("src/10/input.txt").expect("File not found");

    let signal_sum = sum_signal_strengths(input.as_str());

    println!("The signal strength sum is {}", signal_sum);
}

#[derive(Debug)]
enum Op {
    Noop,
    Addx(i64),
}

struct State {
    cycle: i64,
    x: i64,
}

const STOPS: [i64; 6] = [20, 60, 100, 140, 180, 220];

fn sum_signal_strengths(input: &str) -> i64 {
    let mut sum = 0i64;
    let mut state = State::new();

    for line in input.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        match split.get(0) {
            Some(&"noop") => sum += state.do_op(Op::Noop),
            Some(&"addx") => sum += state.do_op(Op::Addx(split.get(1).unwrap().parse().unwrap())),
            _ => panic!("Unexpected op"),
        };
    }

    sum
}

impl State {
    fn new() -> State {
        State { cycle: 0, x: 1 }
    }

    fn do_op(&mut self, op: Op) -> i64 {
        self.cycle += 1;
        let mut signal_strength = self.check_signal_strength();

        if let Op::Addx(add) = op {
            self.cycle += 1;

            if signal_strength == 0 {
                signal_strength = self.check_signal_strength();
            }

            self.x += add;
        }

        signal_strength
    }

    fn check_signal_strength(&self) -> i64 {
        if STOPS.contains(&self.cycle) {
            self.cycle * self.x
        } else {
            0
        }
    }
}

#[test]
fn test() {
    let input = fs::read_to_string("src/10/test.txt").expect("File not found");

    assert_eq!(sum_signal_strengths(input.as_str()), 13140);
}
