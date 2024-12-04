use itertools::Itertools;

pub fn solve_part1(input: &str) -> usize {
    std::iter::successors(Some(input.to_string()), |i| Some(look_and_say(i)))
        .nth(40)
        .unwrap()
        .len()
}

fn look_and_say(input: &str) -> String {
    input.chars().chunk_by(|&c| c).into_iter().fold(
        // output is always bigger than input so we can preallocate at least that!
        String::with_capacity(input.len()),
        |mut output, (c, g)| {
            output.push_str(&g.count().to_string());
            output.push(c);
            output
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("1", "11")]
    #[case("11", "21")]
    #[case("21", "1211")]
    #[case("1211", "111221")]
    #[case("111221", "312211")]
    fn test_part_1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(look_and_say(input), expected);
    }
}
