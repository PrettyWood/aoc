pub fn solve_part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let n_rows = grid.len();
    let n_cols = grid[0].len();

    (0..n_rows)
        .flat_map(|r| (0..n_cols).map(move |c| (r, c)))
        .filter(|&(r, c)| {
            grid[r][c] == '@'
                && neighbours_iter(r, c, n_rows, n_cols)
                    .map(|(n_r, n_c)| grid[n_r][n_c])
                    .filter(|&ch| ch == '@')
                    .count()
                    < 4
        })
        .count()
}

const DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn neighbours_iter(
    r: usize,
    c: usize,
    n_rows: usize,
    n_cols: usize,
) -> impl Iterator<Item = (usize, usize)> {
    DELTAS.iter().filter_map(move |&(dr, dc)| {
        let n_r = r.checked_add_signed(dr)?;
        let n_c = c.checked_add_signed(dc)?;
        (n_r < n_rows && n_c < n_cols).then_some((n_r, n_c))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        13
    )]
    fn test_solve_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part1(input), expected);
    }
}
