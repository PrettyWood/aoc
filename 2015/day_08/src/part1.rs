pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (c_count, mem_count) = count_chars(l);
            c_count - mem_count
        })
        .sum()
}

fn count_chars(string: &str) -> (usize, usize) {
    let c_count = string.len();

    let mut mem_count = 0;
    let mut c_iter = string.trim_matches('"').chars();
    while let Some(c) = c_iter.next() {
        if c == '\\' {
            match c_iter.next() {
                Some('\\') | Some('"') => (),
                Some('x') => {
                    let c1 = c_iter.next().expect("c1 should exist");
                    assert!(c1.is_ascii_hexdigit());
                    let c2 = c_iter.next().expect("c2 should exist");
                    assert!(c2.is_ascii_hexdigit());
                }
                Some(_) => mem_count += 1,
                None => (),
            }
        }
        mem_count += 1;
    }

    (c_count, mem_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(r#""""#, (2, 0))]
    #[case(r#""abc""#, (5, 3))]
    #[case(r#""aaa\"aaa""#, (10, 7))]
    #[case(r#""\x27""#, (6, 1))]
    fn test_part_1(#[case] input: &str, #[case] expected: (usize, usize)) {
        assert_eq!(count_chars(input), expected);
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(
            solve_part1(
                r#"""
"abc"
"aaa\"aaa"
"\x27""#
            ),
            12
        );
    }
}
