pub fn solve_part2(input: &str) -> usize {
    input.lines().map(ribbon_for_line).sum()
}

fn ribbon_for_line(line: &str) -> usize {
    let dims: Vec<usize> = line
        .split("x")
        .map(|dim| dim.parse::<usize>().expect("dimension should be positive"))
        .collect();
    assert_eq!(dims.len(), 3);
    let half_perim =
        dims.iter().sum::<usize>() - dims.iter().max().expect("dims should have a max");
    2 * half_perim + dims.iter().product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("2x3x4", 34)]
    #[case("1x1x10", 14)]
    fn test_ribbon_for_line(#[case] line: &str, #[case] expected: usize) {
        assert_eq!(ribbon_for_line(line), expected);
    }
}
