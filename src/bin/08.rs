use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/08.txt").expect("File not found");

    let num_visible = num_visible_trees(input.as_str()).unwrap();
    let scenic_score = scenic_score(input.as_str()).unwrap();

    println!("Number of visible trees: {}", num_visible);
    println!("Best scenic score: {}", scenic_score);
}

fn num_visible_trees(input: &str) -> Option<u64> {
    let matrix = parse_input(input);

    let mut count = 0u64;
    for y in 1..matrix.len() - 1 {
        for x in 1..matrix.first()?.len() - 1 {
            let height = matrix.get(y)?.get(x)?;
            let mut is_visible = [true, true, true, true];
            let mut i = 0usize;
            for yy in 0..matrix.len() {
                if yy == y {
                    i += 1;
                    continue;
                }
                if matrix.get(yy)?.get(x)? >= height {
                    is_visible[i] = false;
                }
            }
            i += 1;
            for xx in 0..matrix.first()?.len() {
                if xx == x {
                    i += 1;
                    continue;
                }
                if matrix.get(y)?.get(xx)? >= height {
                    is_visible[i] = false;
                }
            }
            let is_visible = is_visible.into_iter().reduce(|acc, item| (acc || item))?;
            count += if is_visible { 1 } else { 0 };
        }
    }

    Some(count + num_outside(&matrix)?)
}

fn scenic_score(input: &str) -> Option<u64> {
    let matrix = parse_input(input);

    let mut max_score = 0u64;
    for y in 1..matrix.len() - 1 {
        for x in 1..matrix.first()?.len() - 1 {
            let height = matrix.get(y)?.get(x)?;
            let mut scores = [0u64; 4];
            for yy in (0..y).rev() {
                scores[0] += 1;
                if matrix.get(yy)?.get(x)? >= height {
                    break;
                }
            }
            for yy in y + 1..matrix.len() {
                scores[1] += 1;
                if matrix.get(yy)?.get(x)? >= height {
                    break;
                }
            }
            for xx in (0..x).rev() {
                scores[2] += 1;
                if matrix.get(y)?.get(xx)? >= height {
                    break;
                }
            }
            for xx in x + 1..matrix.first()?.len() {
                scores[3] += 1;
                if matrix.get(y)?.get(xx)? >= height {
                    break;
                }
            }
            let score = scores.into_iter().reduce(|acc, s| acc * s)?;
            max_score = score.max(max_score);
        }
    }

    Some(max_score)
}

fn num_outside(matrix: &Vec<Vec<u8>>) -> Option<u64> {
    let rows = matrix.len();
    let cols = matrix.first()?.len();

    (2 * rows + 2 * cols - 4).try_into().ok()
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>()
}

#[test]
fn test() {
    let input = "30373\n\
                        25512\n\
                        65332\n\
                        33549\n\
                        35390";

    for x in 2..=0 {
        println!("{x}");
    }

    assert_eq!(num_visible_trees(input), Some(21));
    assert_eq!(scenic_score(input), Some(8));
}
