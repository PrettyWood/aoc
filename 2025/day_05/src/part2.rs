pub fn solve_part2(input: &str) -> usize {
    let (ranges, _ingredients) = input.split_once("\n\n").unwrap();
    let mut ranges: Vec<_> = ranges
        .lines()
        .map(|l| {
            let (start, end) = l.split_once('-').unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            start..=end
        })
        .collect();
    ranges.sort_by_key(|r| *r.start());

    let simplified_ranges: Vec<std::ops::RangeInclusive<usize>> =
        ranges.into_iter().fold(Vec::new(), |mut acc, range| {
            match acc.last_mut() {
                Some(last) if *range.start() <= last.end() + 1 => {
                    *last = *last.start()..=std::cmp::max(*range.end(), *last.end());
                }
                _ => acc.push(range),
            }
            acc
        });

    simplified_ranges
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        14
    )]
    fn test_solve_part2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part2(input), expected);
    }
}
