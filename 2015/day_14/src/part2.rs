use std::collections::HashMap;

use chumsky::prelude::*;
use text::newline;

pub fn solve_part2(input: &str) -> usize {
    winner(input, 2503).1
}

fn winner(input: &str, seconds: usize) -> (String, usize) {
    let reindeers: Vec<Reindeer> = parser().parse(input).expect("should be valid reeinders");
    let mut scores: HashMap<&Reindeer, usize> = reindeers.iter().map(|r| (r, 0)).collect();

    for current_seconds in 1..seconds {
        let max_distance = reindeers
            .iter()
            .map(|r| r.distance(current_seconds))
            .max()
            .unwrap();
        let current_winners: Vec<&Reindeer> = reindeers
            .iter()
            .filter(|r| r.distance(current_seconds) == max_distance)
            .collect();

        for current_winner in &current_winners {
            scores.entry(current_winner).and_modify(|score| *score += 1);
        }
    }

    let (winner, score) = scores
        .into_iter()
        .max_by(|(_, s1), (_, s2)| s1.cmp(s2))
        .unwrap();

    (winner.name.clone(), score)
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Reindeer {
    name: String,
    speed: usize,
    speed_time: usize,
    rest_time: usize,
}

impl Reindeer {
    fn distance(&self, seconds: usize) -> usize {
        let cycle_count = seconds / (self.speed_time + self.rest_time);
        let extra_time = seconds % (self.speed_time + self.rest_time);
        (cycle_count * self.speed_time + extra_time.min(self.speed_time)) * self.speed
    }
}

fn parser() -> impl Parser<char, Vec<Reindeer>, Error = Simple<char>> {
    reindeer_parser().separated_by(newline())
}

// Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
fn reindeer_parser() -> impl Parser<char, Reindeer, Error = Simple<char>> {
    text::ident()
        .then_ignore(just(" can fly "))
        .then(usize_parser())
        .then_ignore(just(" km/s for "))
        .then(usize_parser())
        .then_ignore(just(" seconds, but then must rest for "))
        .then(usize_parser())
        .then_ignore(just(" seconds."))
        .map(|(((name, speed), speed_time), rest_time)| Reindeer {
            name,
            speed,
            speed_time,
            rest_time,
        })
}

fn usize_parser() -> impl Parser<char, usize, Error = Simple<char>> {
    text::int(10).map(|s: String| s.parse::<usize>().expect("should be a valid usize"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winner() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        assert_eq!(winner(input, 1000), ("Dancer".to_string(), 689));
    }
}
