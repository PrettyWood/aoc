pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let (game, sets) = line.split_once(": ").unwrap();
            let id = game
                .strip_prefix("Game ")
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;
            sets.split("; ").for_each(|set_str| {
                let tokens: Vec<_> = set_str.split(", ").collect();
                for token in tokens {
                    let (n, color) = token.split_once(" ").unwrap();
                    let n = n.parse::<usize>().unwrap();
                    match color {
                        "red" => {
                            if n > max_red {
                                max_red = n
                            }
                        }
                        "green" => {
                            if n > max_green {
                                max_green = n
                            }
                        }
                        "blue" => {
                            if n > max_blue {
                                max_blue = n
                            }
                        }
                        _ => panic!("color not supported"),
                    };
                }
            });

            (max_red <= 12 && max_green <= 13 && max_blue <= 14).then_some(id)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(solve_part1(input), 8);
    }
}
