use itertools::Itertools;

pub fn solve_part2(input: &str) -> usize {
    let (left_col, right_col): (Vec<usize>, Vec<usize>) = input
        .lines()
        .map(|l| {
            let (l, r) = l.split_once("   ").expect("there should be two cols");
            (
                l.parse::<usize>().expect("l should be a number"),
                r.parse::<usize>().expect("r should be a number"),
            )
        })
        .collect();

    let right_col_counts = right_col.iter().counts();

    left_col
        .iter()
        .map(|n| n * right_col_counts.get(n).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(solve_part2(input), 31);
    }
}
