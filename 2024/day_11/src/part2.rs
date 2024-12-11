use dashmap::DashMap;
use rayon::prelude::*;
use std::sync::Arc;

pub fn solve_part2(input: &str) -> usize {
    solve(input, 75)
}

type Cache = DashMap<(usize, usize), usize>;

fn solve(input: &str, turn: usize) -> usize {
    let cache: Arc<Cache> = Arc::new(DashMap::new());
    input
        .split_whitespace()
        .par_bridge()
        .map(|v| solve_one_with_cache(v.parse::<usize>().unwrap(), turn, &cache))
        .sum()
}

fn solve_one_with_cache(value: usize, turn: usize, cache: &Cache) -> usize {
    if turn == 0 {
        return 1;
    }

    if let Some(count) = cache.get(&(value, turn)) {
        return *count;
    }

    let s = value.to_string();
    let count = match value {
        0 => solve_one_with_cache(1, turn - 1, cache),
        _ if s.len() % 2 == 0 => {
            let (left, right) = s.split_at(s.len() / 2);
            solve_one_with_cache(left.parse::<usize>().unwrap(), turn - 1, cache)
                + solve_one_with_cache(right.parse::<usize>().unwrap(), turn - 1, cache)
        }
        v => solve_one_with_cache(v * 2024, turn - 1, cache),
    };

    cache.insert((value, turn), count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve("125 17", 6), 22);
        assert_eq!(solve("125 17", 25), 55312);
    }
}
