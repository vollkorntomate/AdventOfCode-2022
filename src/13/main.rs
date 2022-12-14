use std::{
    cmp::Ordering,
    fs,
    iter::{Peekable, Skip},
    str::Chars,
};

type Pair = (PacketItem, PacketItem);

fn main() {
    let input = fs::read_to_string("src/13/input.txt").unwrap();

    let indices_sum = sum_indices(input.as_str());

    println!("The indices sum is {}", indices_sum);
}

#[derive(Debug)]
enum PacketItem {
    Number(u64),
    List(Vec<PacketItem>),
}

fn sum_indices(input: &str) -> u64 {
    let pairs = parse_input(input);

    pairs
        .into_iter()
        .enumerate()
        .map(|(i, (left, right))| {
            if compare(&left, &right) == Ordering::Less {
                i as u64 + 1
            } else {
                0
            }
        })
        .sum()
}

fn compare(left: &PacketItem, right: &PacketItem) -> Ordering {
    match (left, right) {
        (PacketItem::Number(l), PacketItem::Number(r)) => l.cmp(r),
        (PacketItem::List(_), PacketItem::Number(r)) => {
            compare(left, &PacketItem::List(vec![PacketItem::Number(*r)]))
        }
        (PacketItem::Number(r), PacketItem::List(_)) => {
            compare(&PacketItem::List(vec![PacketItem::Number(*r)]), right)
        }
        (PacketItem::List(l), PacketItem::List(r)) => {
            for i in 0..l.len() {
                if let None = r.get(i) {
                    return Ordering::Greater;
                }
                let res = compare(&l[i], &r[i]);
                if res != Ordering::Equal {
                    return res;
                }
            }
            if r.len() > l.len() {
                return Ordering::Less;
            }
            Ordering::Equal
        }
    }
}

fn parse_input(input: &str) -> Vec<Pair> {
    input
        .split("\n\n")
        .map(|two_line| {
            let mut split = two_line.split("\n");
            let (left, right) = (split.next().unwrap(), split.next().unwrap());

            (
                parse_list(&mut left.chars().skip(1).peekable()),
                parse_list(&mut right.chars().skip(1).peekable()),
            )
        })
        .collect::<Vec<_>>()
}

fn parse_list(chars: &mut Peekable<Skip<Chars>>) -> PacketItem {
    let mut items = Vec::<PacketItem>::new();
    while let Some(char) = chars.next() {
        if char == '[' {
            items.push(parse_list(chars));
        } else if char == ',' {
            continue;
        } else if char == ']' {
            break;
        } else {
            let mut s = String::from(char);
            while let Some(next) = chars.peek() {
                if !('0'..='9').contains(next) {
                    break;
                }
                s.push(chars.next().unwrap());
            }
            items.push(PacketItem::Number(s.parse::<u64>().unwrap()));
        }
    }

    PacketItem::List(items)
}

#[test]
fn test() {
    let input = "[1,1,3,1,1]\n\
                        [1,1,5,1,1]\n\
                        \n\
                        [[1],[2,3,4]]\n\
                        [[1],4]\n\
                        \n\
                        [9]\n\
                        [[8,7,6]]\n\
                        \n\
                        [[4,4],4,4]\n\
                        [[4,4],4,4,4]\n\
                        \n\
                        [7,7,7,7]\n\
                        [7,7,7]\n\
                        \n\
                        []\n\
                        [3]\n\
                        \n\
                        [[[]]]\n\
                        [[]]\n\
                        \n\
                        [1,[2,[3,[4,[5,6,7]]]],8,9]\n\
                        [1,[2,[3,[4,[5,6,0]]]],8,9]";

    assert_eq!(sum_indices(input), 13);
}
