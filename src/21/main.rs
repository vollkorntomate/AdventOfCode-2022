use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("src/21/input.txt").unwrap();

    let root_yell = calc_root_yell(input.as_str());

    println!("The root monkey yells the number {root_yell}");
}

type YellMap<'a> = HashMap<&'a str, Yell<'a>>;

fn calc_root_yell<'a>(input: &'a str) -> i64 {
    let yells = parse(input);

    calc_yell_rec(&yells, "root")
}

fn calc_yell_rec(yells: &YellMap, monkey: &str) -> i64 {
    match yells.get(monkey).unwrap() {
        Yell::Number(num) => *num,
        Yell::Op((left_monkey, right_monkey, op)) => {
            let left = calc_yell_rec(yells, left_monkey);
            let right = calc_yell_rec(yells, right_monkey);

            match op {
                '+' => left + right,
                '-' => left - right,
                '*' => left * right,
                '/' => left / right,
                _ => panic!("Unexpected operator"),
            }
        }
    }
}

fn parse<'a>(input: &'a str) -> YellMap {
    let mut yells = YellMap::new();

    for line in input.lines().filter(|&l| !l.trim().is_empty()) {
        let monkey_name = &line[0..4];
        let mut split = line.split(" ").skip(1);

        let yell_str = split.next().unwrap();
        let yell = match yell_str.parse::<i64>() {
            Ok(num) => Yell::Number(num),
            Err(_) => {
                let first = yell_str;
                let op = split.next().unwrap().chars().next().unwrap();
                let second = split.next().unwrap();

                Yell::Op((first, second, op))
            }
        };

        yells.insert(monkey_name, yell);
    }

    yells
}

enum Yell<'a> {
    Number(i64),
    Op((&'a str, &'a str, char)),
}

#[test]
fn test() {
    let input = "root: pppw + sjmn\n\
                    dbpl: 5\n\
                    cczh: sllz + lgvd\n\
                    zczc: 2\n\
                    ptdq: humn - dvpt\n\
                    dvpt: 3\n\
                    lfqf: 4\n\
                    humn: 5\n\
                    ljgn: 2\n\
                    sjmn: drzm * dbpl\n\
                    sllz: 4\n\
                    pppw: cczh / lfqf\n\
                    lgvd: ljgn * ptdq\n\
                    drzm: hmdt - zczc\n\
                    hmdt: 32";

    assert_eq!(calc_root_yell(input), 152);
}
