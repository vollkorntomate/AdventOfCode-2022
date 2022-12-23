use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/02.txt").expect("File not found");

    let score = score(input.as_str());

    println!("The score is {score}.");
}

fn score(input: &str) -> u64 {
    input
        .lines()
        .into_iter()
        .map(parse_line)
        .map(|(own, outcome)| own.score(&outcome))
        .sum()
}

fn parse_line(line: &str) -> (RPS, Outcome) {
    let strat: Vec<&str> = line.trim().split(" ").collect();
    let other = RPS::parse(strat.get(0).unwrap());
    let outcome = Outcome::parse(strat.get(1).unwrap());
    let own = other.needed(&outcome);

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
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => Self::Scissors,
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

    fn needed(self, outcome: &Outcome) -> RPS {
        match (&self, outcome) {
            (RPS::Rock, Outcome::Win) => RPS::Paper,
            (RPS::Rock, Outcome::Lose) => RPS::Scissors,
            (RPS::Paper, Outcome::Win) => RPS::Scissors,
            (RPS::Paper, Outcome::Lose) => RPS::Rock,
            (RPS::Scissors, Outcome::Win) => RPS::Rock,
            (RPS::Scissors, Outcome::Lose) => RPS::Paper,
            (_, Outcome::Draw) => self,
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
    fn parse(input: &str) -> Outcome {
        match input {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Illegal value"),
        }
    }

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
    assert_eq!(score(input), 12);
}
