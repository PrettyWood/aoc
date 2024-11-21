pub fn solve_part2(secret_key: &str) -> usize {
    (1..)
        .filter_map(|n| {
            let digest = md5::compute(format!("{secret_key}{n}"));
            format!("{digest:?}").starts_with("000000").then(|| n)
        })
        .next()
        .expect("should have at least one element")
}
