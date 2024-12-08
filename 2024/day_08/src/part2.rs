use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn solve_part2(input: &str) -> usize {
    Grid::from(input).get_all_unique_antinodes().len()
}

#[derive(Debug)]
struct Grid {
    n_rows: usize,
    n_cols: usize,
    antennas: Antennas,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();
        let antennas: Antennas = lines
            .iter()
            .enumerate()
            .flat_map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| c.is_ascii_alphanumeric())
                    .map(move |(j, c)| (c, (i as isize, j as isize)))
            })
            .fold(Antennas::new(), |mut antennas, (fq, pos)| {
                antennas.entry(fq).or_default().push(pos);
                antennas
            });

        Grid {
            n_rows: lines.len(),
            n_cols: lines[0].len(),
            antennas,
        }
    }
}

impl Grid {
    fn is_inside(&self, (x, y): Position) -> bool {
        x >= 0 && x < self.n_rows as isize && y >= 0 && y < self.n_cols as isize
    }

    fn get_antinodes(&self, fq: &Frequency) -> HashSet<Position> {
        let positions = self.antennas.get(fq).expect("fq should be in map");
        positions
            .iter()
            .combinations(2)
            .flat_map(|pos1_pos2| {
                let (x1, y1) = *pos1_pos2[0];
                let (x2, y2) = *pos1_pos2[1];

                let dx = x1.abs_diff(x2) as isize;
                let dy = y1.abs_diff(y2) as isize;

                let (mx1, my1, mx2, my2) = if x1 <= x2 && y1 <= y2 {
                    // ....A........
                    // ........B....
                    (-1, -1, 1, 1)
                } else if x1 <= x2 && y1 > y2 {
                    // ........A....
                    // ....B........
                    (-1, 1, 1, -1)
                } else if x1 > x2 && y1 <= y2 {
                    // ........B....
                    // ....A........
                    (1, -1, -1, 1)
                } else {
                    // ....B.......
                    // ........A...
                    (1, 1, -1, -1)
                };

                (0..)
                    .map(move |i| (x1 + i * mx1 * dx, y1 + i * my1 * dy))
                    .take_while(|&pos| self.is_inside(pos))
                    .chain(
                        (0..)
                            .map(move |i| (x2 + i * mx2 * dx, y2 + i * my2 * dy))
                            .take_while(|&pos| self.is_inside(pos)),
                    )
            })
            .collect()
    }

    fn get_all_unique_antinodes(&self) -> HashSet<Position> {
        self.antennas
            .keys()
            .fold(HashSet::new(), |mut unique_antinodes_pos, fq| {
                unique_antinodes_pos.extend(self.get_antinodes(fq));
                unique_antinodes_pos
            })
    }
}

type Position = (isize, isize);
type Frequency = char;
type Antennas = HashMap<Frequency, Vec<Position>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_antinodes() {
        let input = "T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........";
        let grid: Grid = input.into();
        let antinodes_pos: HashSet<Position> = input
            .lines()
            .enumerate()
            .flat_map(|(i, l)| {
                l.chars().enumerate().filter_map(move |(j, c)| {
                    (matches!(c, 'T' | '#')).then_some((i as isize, j as isize))
                })
            })
            .collect();
        assert_eq!(grid.get_antinodes(&'T'.into()), antinodes_pos);
    }

    #[test]
    fn test_solve_part2() {
        let input = "##....#....#
.#.#....0...
..#.#0....#.
..##...0....
....0....#..
.#...#A....#
...#..#.....
#....#.#....
..#.....A...
....#....A..
.#........#.
...#......##";
        assert_eq!(solve_part2(input), 34);
    }
}
