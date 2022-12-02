use std::fs;

fn main() {
    let input = fs::read_to_string("src/02/input.txt").expect("File not found");

    let score = score(input.as_str());

    println!("The score is {score}.");
}

fn score(input: &str) -> u64 {
    input
        .lines()
        .into_iter()
        .map(parse_line)
        .map(|(own, outcome)| {
            own.score(&outcome)
        })
        .sum()
}

fn parse_line(line: &str) -> (RPS, Outcome) {
    let strat: Vec<&str> = line.trim().split(" ").collect();
    let own = RPS::parse(strat.get(1).unwrap());
    let other = &RPS::parse(strat.get(0).unwrap());
    let outcome = own.outcome(other);

    (own, outcome)
}

#[derive(Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn parse(input: &str) -> RPS {
        match input {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Illegal value"),
        }
    }

    fn score(&self, outcome: &Outcome) -> u64 {
        let score: u64 = match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };

        score + outcome.score()
    }

    fn outcome(&self, other: &RPS) -> Outcome {
        match (self, other) {
            (RPS::Rock, RPS::Paper) => Outcome::Lose,
            (RPS::Rock, RPS::Scissors) => Outcome::Win,
            (RPS::Paper, RPS::Rock) => Outcome::Win,
            (RPS::Paper, RPS::Scissors) => Outcome::Lose,
            (RPS::Scissors, RPS::Rock) => Outcome::Lose,
            (RPS::Scissors, RPS::Paper) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn score(&self) -> u64 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

#[test]
fn test() {
    let input = "A Y\nB X\nC Z";
    assert_eq!(score(input), 15);
}
