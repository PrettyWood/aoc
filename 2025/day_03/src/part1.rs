pub fn solve_part1(input: &str) -> usize {
    input.lines().map(largest_joltage).sum()
}

fn largest_joltage(input: &str) -> usize {
    let bytes = input.as_bytes();

    for tens in (1..=9).rev() {
        if let Some(pos) = bytes.iter().position(|&b| b == b'0' + tens) {
            for units in (0..=9).rev() {
                if bytes[pos + 1..].contains(&(b'0' + units)) {
                    return (tens * 10 + units) as usize;
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    fn test_largest_joltage(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(largest_joltage(input), expected);
    }

    #[rstest]
    #[case(
        "987654321111111
811111111111119
234234234234278
818181911112111",
        357
    )]
    fn test_solve_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part1(input), expected);
    }
}
