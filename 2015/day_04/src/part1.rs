pub fn solve_part1(secret_key: &str) -> usize {
    (1..)
        .filter_map(|n| {
            let digest = md5::compute(format!("{secret_key}{n}"));
            format!("{digest:?}").starts_with("00000").then(|| n)
        })
        .next()
        .expect("should have at least one element")
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abcdef", 609043)]
    #[case("pqrstuv", 1048970)]
    fn test_solve_part1(#[case] secret_key: &str, #[case] expected: usize) {
        assert_eq!(solve_part1(secret_key), expected);
    }
}
