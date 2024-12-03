pub fn solve_part2(input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("", "")]
    fn test_solve_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(input, expected);
    }
}
