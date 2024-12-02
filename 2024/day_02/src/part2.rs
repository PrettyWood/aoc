pub fn solve_part2(input: &str) -> usize {
    input.lines().filter(|l| is_safe_line(l)).count()
}

fn is_safe_line(line: &str) -> bool {
    let values: Vec<_> = line
        .split_whitespace()
        .map(|v| v.parse::<u8>().expect("line should be a list of numbers"))
        .collect();

    (0..values.len()).any(|i| is_safe(&[&values[..i], &values[i + 1..]].concat()))
}

fn is_safe(values: &[u8]) -> bool {
    values
        .iter()
        .zip(values.iter().skip(1))
        .all(|(v1, v2)| v2 > v1 && v2 - v1 <= 3)
        || values
            .iter()
            .zip(values.iter().skip(1))
            .all(|(v1, v2)| v1 > v2 && v1 - v2 <= 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("7 6 4 2 1", true)]
    #[case("1 2 7 8 9", false)]
    #[case("9 7 6 2 1", false)]
    #[case("1 3 2 4 5", true)]
    #[case("8 6 4 4 1", true)]
    #[case("1 3 6 7 9", true)]
    fn test_is_safe_line(#[case] line: &str, #[case] is_safe: bool) {
        assert_eq!(is_safe_line(line), is_safe);
    }
}
