use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/11.txt").unwrap();

    let monkey_business = play(input.as_str());

    println!("Monkey Business is {}", monkey_business);
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let blocks = input.split("\n\n").collect::<Vec<_>>();
    blocks
        .into_iter()
        .map(Monkey::parse)
        .map(|m| m.unwrap())
        .collect::<Vec<_>>()
}

fn play(input: &str) -> u64 {
    let mut monkeys = parse_input(input);
    let mut inspections = (0..monkeys.len()).map(|_| 0u64).collect::<Vec<_>>();
    let total_mod = monkeys.iter().map(|m| m.test).product::<u64>();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            for item in monkey.items {
                inspections[i] += 1;
                monkeys[i].items.remove(0);

                let worry = monkey.op.perform(item) % total_mod;
                if worry % monkey.test == 0 {
                    monkeys[monkey.action.0].items.push(worry);
                } else {
                    monkeys[monkey.action.1].items.push(worry);
                }
            }
        }
    }

    inspections.sort();
    inspections.reverse();

    inspections[0] * inspections[1]
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Op,
    test: u64,
    action: (usize, usize),
}

impl Monkey {
    fn parse(input: &str) -> Option<Monkey> {
        let lines = input.lines().skip(1).map(|l| l.trim()).collect::<Vec<_>>();

        let monkey = Monkey {
            items: lines.get(0)?["Starting items: ".len()..]
                .split(", ")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>(),
            op: {
                let op_str = &lines.get(1)?["Operation: new = old ".len()..];
                if op_str.contains("old") {
                    Op::Square
                } else if op_str.starts_with("*") {
                    Op::Mul(op_str[1..].trim().parse().unwrap())
                } else {
                    Op::Add(op_str[1..].trim().parse().unwrap())
                }
            },
            test: lines.get(2)?["Test: divisible by ".len()..]
                .trim()
                .parse()
                .unwrap(),
            action: (
                lines.get(3)?["If true: throw to mokey ".len()..]
                    .trim()
                    .parse()
                    .unwrap(),
                lines.get(4)?["If false: throw to mokey ".len()..]
                    .trim()
                    .parse()
                    .unwrap(),
            ),
        };

        Some(monkey)
    }
}

#[derive(Clone, Debug)]
enum Op {
    Add(u64),
    Mul(u64),
    Square,
}

impl Op {
    fn perform(&self, init: u64) -> u64 {
        match self {
            Op::Add(v) => init + v,
            Op::Mul(v) => init * v,
            Op::Square => init * init,
        }
    }
}

#[test]
fn test() {
    let input = fs::read_to_string("test-inputs/11").unwrap();

    assert_eq!(play(input.as_str()), 2713310158);
}
