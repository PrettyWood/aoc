use itertools::Itertools;

pub fn solve_part2(password: &str) -> String {
    std::iter::successors(Some(password.to_string()), |p| Some(next_string(p)))
        .filter(|p| is_safe_password(p))
        .nth(1)
        .unwrap()
        .to_string()
}

fn next_string(string: &str) -> String {
    if let Some(idx) = string.as_bytes().iter().rposition(|&c| c != b'z') {
        string[..idx].to_string()
            + &((string.as_bytes()[idx] + 1) as char).to_string()
            + &"a".repeat(string.len() - idx - 1)
    } else {
        "a".repeat(string.len() + 1)
    }
}

fn is_safe_password(password: &str) -> bool {
    let has_three_chars_in_row = password
        .as_bytes()
        .iter()
        .tuple_windows::<(_, _, _)>()
        .any(|(&c1, &c2, &c3)| c1 + 2 == c2 + 1 && c2 + 1 == c3);

    let has_no_invalid_char = password
        .as_bytes()
        .iter()
        .all(|c| !matches!(c, b'i' | b'o' | b'l'));

    let pairs: Vec<usize> = password
        .as_bytes()
        .iter()
        .tuple_windows::<(_, _)>()
        .enumerate()
        .filter_map(|(i, (&c1, &c2))| (c1 == c2).then_some(i))
        .collect();

    has_three_chars_in_row
        && has_no_invalid_char
        && pairs.len() >= 2
        && (pairs.len() > 3 || (pairs[1] - pairs[0] > 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abcdefgh", "abcdffaa")]
    #[case("ghijklmn", "ghjaabcc")]
    fn test_solve_part2(#[case] password: &str, #[case] next_safe: &str) {
        assert_eq!(solve_part2(password), next_safe);
    }

    #[rstest]
    #[case("a", "b")]
    #[case("z", "aa")]
    #[case("xz", "ya")]
    #[case("zyz", "zza")]
    #[case("zzzzzz", "aaaaaaa")]
    fn test_next_string(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(next_string(input), expected);
    }

    #[rstest]
    #[case("hijklmmn", false)]
    #[case("abbceffg", false)]
    #[case("abbcegjk", false)]
    #[case("abcdffaa", true)]
    #[case("ghjaabcc", true)]
    #[case("abcdeggg", false)]
    fn test_is_safe_password(#[case] password: &str, #[case] is_safe: bool) {
        assert_eq!(is_safe_password(password), is_safe);
    }
}
