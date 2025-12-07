pub fn solve_part1(input: &str) -> usize {
    let (values, ops_line) = input.rsplit_once('\n').unwrap();
    let values: Vec<Vec<usize>> = values
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    ops_line
        .split_whitespace()
        .enumerate()
        .map(|(col, op_str)| {
            values
                .iter()
                .map(|row| row[col])
                .reduce(match op_str {
                    "+" => std::ops::Add::add,
                    "*" => std::ops::Mul::mul,
                    _ => unreachable!("only + and * are supported"),
                })
                .expect("")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        "123 328  51 64
45 64  387 23
6 98  215 314
*   +   *   +",
        4277556
    )]
    fn test_solve_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part1(input), expected);
    }
}
