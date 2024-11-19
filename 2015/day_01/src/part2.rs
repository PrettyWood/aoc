pub fn solve_part2(input: &str) -> usize {
    input
        .chars()
        .scan(0, |floor, c| {
            *floor = match c {
                '(' => *floor + 1,
                ')' => *floor - 1,
                _ => panic!("invalid character {c:?}"),
            };
            Some(*floor)
        })
        .position(|floor| floor == -1)
        .expect("there should be at least one position for -1")
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(")", 1)]
    #[case("()())", 5)]
    #[case("()())((((", 5)]
    fn test_solve_part2(#[case] input: &str, #[case] expected_position: usize) {
        assert_eq!(solve_part2(input), expected_position)
    }
}
