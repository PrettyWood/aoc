pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (c_count, encoded_count) = count_chars(l);
            encoded_count - c_count
        })
        .sum()
}

fn count_chars(string: &str) -> (usize, usize) {
    let c_count = string.len();

    // we start at 2 because we'll add the two quotes around
    let encoded_count = string.as_bytes().iter().fold(2_usize, |acc, byte| {
        if matches!(byte, b'"' | b'\\') {
            acc + 2
        } else {
            acc + 1
        }
    });

    (c_count, encoded_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(r#""""#, (2, 6))]
    #[case(r#""abc""#, (5, 9))]
    #[case(r#""aaa\"aaa""#, (10, 16))]
    #[case(r#""\x27""#, (6, 11))]
    fn test_part_2(#[case] input: &str, #[case] expected: (usize, usize)) {
        assert_eq!(count_chars(input), expected);
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            solve_part2(
                r#"""
"abc"
"aaa\"aaa"
"\x27""#
            ),
            19
        );
    }
}
