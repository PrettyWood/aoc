pub fn solve_part2(secret_key: &str) -> usize {
    (1..)
        .find(|n| {
            let digest = md5::compute(format!("{secret_key}{n}"));
            format!("{digest:?}").starts_with(&"0".repeat(6))
        })
        .expect("should have at least one element")
}
