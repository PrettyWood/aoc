pub fn solve_part1(input: &str) -> usize {
    let (mut left_col, mut right_col): (Vec<usize>, Vec<usize>) = input
        .lines()
        .map(|l| {
            let (l, r) = l.split_once("   ").expect("there should be two cols");
            (
                l.parse::<usize>().expect("l should be a number"),
                r.parse::<usize>().expect("r should be a number"),
            )
        })
        .collect();

    left_col.sort();
    right_col.sort();

    left_col
        .iter()
        .zip(right_col.iter())
        .map(|(l, r)| r.abs_diff(*l))
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(solve_part1(input), 11);
    }
}
