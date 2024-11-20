pub fn solve_part1(input: &str) -> usize {
    input.lines().map(wrapping_paper_for_line).sum()
}

fn wrapping_paper_for_line(line: &str) -> usize {
    let dims: Vec<usize> = line
        .split("x")
        .map(|dim| dim.parse::<usize>().expect("dimension should be positive"))
        .collect();
    assert_eq!(dims.len(), 3);
    let areas = vec![dims[0] * dims[1], dims[1] * dims[2], dims[2] * dims[0]];
    2 * areas.iter().sum::<usize>() + areas.iter().min().expect("dims should not be empty")
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("2x3x4", 58)]
    #[case("1x1x10", 43)]
    fn test_wrapping_paper_for_line(#[case] line: &str, #[case] expected: usize) {
        assert_eq!(wrapping_paper_for_line(line), expected);
    }
}
