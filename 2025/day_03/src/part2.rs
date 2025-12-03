pub fn solve_part2(input: &str) -> usize {
    input.lines().map(largest_joltage).sum()
}

fn largest_joltage(input: &str) -> usize {
    let bytes = input.as_bytes();

    let (_, total) = (0..12)
        .rev()
        .fold((0, 0_usize), |(start, acc), remaining_digits| {
            let end = bytes.len() - remaining_digits;
            let (digit, relative_pos) = (0..=9)
                .rev()
                .find_map(|digit| {
                    bytes[start..end]
                        .iter()
                        .position(|&b| b == b'0' + digit)
                        .map(|pos| (digit, pos))
                })
                .unwrap();
            (start + relative_pos + 1, acc * 10 + digit as usize)
        });

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    fn test_largest_joltage(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(largest_joltage(input), expected);
    }

    #[rstest]
    #[case(
        "987654321111111
811111111111119
234234234234278
818181911112111",
        3121910778619
    )]
    fn test_solve_part2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part2(input), expected);
    }
}
