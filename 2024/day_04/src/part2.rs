const AROUND_DPOS: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];
const VALID_M_AROUND_DPOS_IDX: [(usize, usize); 4] = [(0, 1), (0, 3), (1, 2), (2, 3)];

pub fn solve_part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let n_cols = grid[0].len();
    let n_rows = grid.len();

    (1..n_rows - 1)
        .flat_map(|i| {
            (1..n_cols - 1)
                .map(move |j| (i, j))
                .filter(|&(i, j)| grid[i][j] == 'A')
                .flat_map(|(a_i, a_j)| {
                    VALID_M_AROUND_DPOS_IDX
                        .iter()
                        .map(move |&(m1_dpos_idx, m2_dpos_idx)| {
                            (a_i, a_j, m1_dpos_idx, m2_dpos_idx)
                        })
                })
                .filter(|&(a_i, a_j, m1_dpos_idx, m2_dpos_idx)| {
                    let (m1_di, m1_dj) = AROUND_DPOS[m1_dpos_idx];
                    let (m2_di, m2_dj) = AROUND_DPOS[m2_dpos_idx];

                    grid[get_idx(a_i, m1_di)][get_idx(a_j, m1_dj)] == 'M'
                        && grid[get_idx(a_i, m2_di)][get_idx(a_j, m2_dj)] == 'M'
                        && (0..4)
                            .filter(|&dpos_idx| dpos_idx != m1_dpos_idx && dpos_idx != m2_dpos_idx)
                            .all(|dpos_idx| {
                                let (di, dj) = AROUND_DPOS[dpos_idx];
                                grid[get_idx(a_i, di)][get_idx(a_j, dj)] == 'S'
                            })
                })
        })
        .count()
}

fn get_idx(base: usize, delta: i8) -> usize {
    let res = base as isize - delta as isize;
    assert!(res >= 0);
    res as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part2() {
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
        assert_eq!(solve_part2(input), 9);
    }
}
