use itertools::*;
use std::collections::{BTreeSet, HashMap};

pub fn solve_part2(input: &str) -> usize {
    let (cities, cities_indices, distances) = build_cities_and_distances(input);

    cities
        .iter()
        .permutations(cities.len())
        .map(|travel| {
            travel
                .iter()
                .tuple_windows::<(_, _)>()
                .fold(0_usize, |distance, (from, to)| {
                    distance + distances[cities_indices[**from]][cities_indices[**to]]
                })
        })
        .max()
        .expect("max should exist")
}

fn build_cities_and_distances(input: &str) -> (Vec<&str>, HashMap<&str, usize>, Vec<Vec<usize>>) {
    //         London, Dublin, Belfast
    // London  (0, 464, 518)
    // Dublin  (464, 0, 141 )
    // Belfast (518, 141, 0)
    //
    // cities = ["London", "Dublin", "Belfast"]
    // distances = [[0, 464, 518 ], ...]
    let mut cities: BTreeSet<&str> = BTreeSet::new();
    let mut distances_vec: Vec<(&str, &str, usize)> = vec![];
    for line in input.lines() {
        let (from_to, distance_str) = line.split_once(" = ").expect("` = ` should be in line");
        let distance = distance_str
            .parse::<usize>()
            .expect("distance should be a number");
        let (from, to) = from_to
            .split_once(" to ")
            .expect("` to ` should be in line");
        cities.insert(from);
        cities.insert(to);
        distances_vec.push((from, to, distance));
    }

    let cities: Vec<&str> = cities.into_iter().collect();
    let cities_indices: HashMap<&str, usize> = cities
        .iter()
        .enumerate()
        .map(|(i, city)| (*city, i))
        .collect();

    let mut distances = vec![vec![0; cities.len()]; cities.len()];
    for &(from, to, distance) in &distances_vec {
        let from_i = cities_indices.get(from).expect("from should be in hashmap");
        let to_i = cities_indices.get(to).expect("to should be in hashmap");
        distances[*from_i][*to_i] = distance;
        distances[*to_i][*from_i] = distance;
    }

    (cities, cities_indices, distances)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_2() {
        let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
        assert_eq!(solve_part2(input), 982);
    }
}
