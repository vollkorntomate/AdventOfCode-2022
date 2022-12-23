use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("inputs/21.txt").unwrap();

    let human_yell = calc_human_yell(input.as_str());

    println!("You have to yell {human_yell}");
}

type YellMap<'a> = HashMap<&'a str, Yell<'a>>;

fn calc_human_yell<'a>(input: &'a str) -> i64 {
    let yells = parse(input);

    let root = yells.get("root").unwrap();
    if let Yell::Op((left, right, _)) = root {
        let root_left = calc_yell_rec(&yells, left);
        let root_right = calc_yell_rec(&yells, right);

        if let Some(search) = root_left {
            return calc_human_yell_rec(&yells, right, search);
        } else {
            return calc_human_yell_rec(&yells, left, root_right.unwrap());
        }
    }
    0
}

fn calc_yell_rec(yells: &YellMap, monkey: &str) -> Option<i64> {
    if monkey == "humn" {
        return None;
    }
    match yells.get(monkey).unwrap() {
        Yell::Number(num) => Some(*num),
        Yell::Op((left_monkey, right_monkey, op)) => {
            let left = calc_yell_rec(yells, left_monkey);
            let right = calc_yell_rec(yells, right_monkey);

            match op {
                '+' => Some(left? + right?),
                '-' => Some(left? - right?),
                '*' => Some(left? * right?),
                '/' => Some(left? / right?),
                _ => panic!("Unexpected operator"),
            }
        }
    }
}

fn calc_human_yell_rec(yells: &YellMap, monkey: &str, search: i64) -> i64 {
    if let Yell::Op((left, right, op)) = yells.get(monkey).unwrap() {
        let left_val = calc_yell_rec(yells, left);
        let right_val = calc_yell_rec(yells, right);

        if let Some(val) = left_val {
            return match op {
                '+' => calc_human_yell_rec(yells, right, search - val),
                '-' => calc_human_yell_rec(yells, right, val - search),
                '*' => calc_human_yell_rec(yells, right, search / val),
                '/' => calc_human_yell_rec(yells, right, val / search),
                _ => panic!("Unexpected operator"),
            };
        } else if let Some(val) = right_val {
            return match op {
                '+' => calc_human_yell_rec(yells, left, search - val),
                '-' => calc_human_yell_rec(yells, left, search + val),
                '*' => calc_human_yell_rec(yells, left, search / val),
                '/' => calc_human_yell_rec(yells, left, search * val),
                _ => panic!("Unexpected operator"),
            };
        } else {
            // None, only for "humn"
            println!("None found for monkey {monkey}, seach = {search}");
            return search;
        }
    }

    search
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

    assert_eq!(calc_human_yell(input), 301);
}
