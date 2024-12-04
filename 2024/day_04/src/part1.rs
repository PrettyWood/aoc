const DIRECTIONS: [(i8, i8); 8] = [
    (0, 1),   // right
    (0, -1),  // left
    (1, 0),   // down
    (-1, 0),  // up
    (1, 1),   // right-down
    (-1, 1),  // right-up
    (1, -1),  // left-down
    (-1, -1), // left-up
];

pub fn solve_part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let n_cols = grid[0].len();
    let n_rows = grid.len();

    (0..n_rows)
        .flat_map(|i| {
            (0..n_cols)
                .map(move |j| (i, j))
                .filter(|&(i, j)| grid[i][j] == 'X')
                .flat_map(|(x_i, x_j)| DIRECTIONS.iter().map(move |&(di, dj)| (x_i, x_j, di, dj)))
                .filter(|&(x_i, x_j, di, dj)| {
                    let other_chars_pos = (1..=3).map(|u| {
                        (
                            x_i as isize + di as isize * u,
                            x_j as isize + dj as isize * u,
                        )
                    });
                    other_chars_pos.clone().all(|(i, j)| {
                        i >= 0 && i < n_rows as isize && j >= 0 && j < n_cols as isize
                    }) && other_chars_pos
                        .map(|(i, j)| grid[i as usize][j as usize])
                        .collect::<String>()
                        == "MAS"
                })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(solve_part1(input), 18);
    }
}
