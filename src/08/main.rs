use std::fs;

fn main() {
    let input = fs::read_to_string("src/08/input.txt").expect("File not found");

    let num_visible = num_visible_trees(input.as_str()).unwrap();

    println!("Number of visible trees: {}", num_visible);
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

    assert_eq!(num_visible_trees(input), Some(21));
}
