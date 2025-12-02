pub fn solve_part2(input: &str) -> usize {
    input
        .split(',')
        .flat_map(|t| {
            let (l, r) = t.split_once('-').unwrap();
            let l: usize = l.parse().unwrap();
            let r: usize = r.parse().unwrap();
            l..=r
        })
        .filter(|n| is_invalid(*n))
        .sum()
}

fn is_invalid(n: usize) -> bool {
    let s = n.to_string();
    (1..=(s.len() / 2)).any(|sub_len| {
        s.as_bytes()
            .chunks(sub_len)
            .all(|chunk| *chunk == s.as_bytes()[..sub_len])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(11, true)]
    #[case(20, false)]
    #[case(22, true)]
    #[case(1010, true)]
    #[case(12341234, true)]
    #[case(123123123, true)]
    #[case(1212121212, true)]
    #[case(1111111, true)]
    fn test_is_invalid(#[case] input: usize, #[case] expected: bool) {
        assert_eq!(is_invalid(input), expected);
    }

    #[rstest]
    #[case("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124", 4174379265)]
    fn test_solve_part2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part2(input), expected);
    }
}
