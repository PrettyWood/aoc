pub fn solve_part1(input: &str) -> isize {
    input.chars().fold(0, |floor, c| match c {
        '(' => floor + 1,
        ')' => floor - 1,
        _ => panic!("invalid character {c:?}"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("(())", 0)]
    #[case("(((", 3)]
    #[case("())", -1)]
    #[case(")())())", -3)]
    fn test_solve_part1(#[case] input: &str, #[case] expected_floor: isize) {
        assert_eq!(solve_part1(input), expected_floor)
    }
}
