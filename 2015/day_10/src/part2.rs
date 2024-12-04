use itertools::Itertools;

pub fn solve_part2(input: &str) -> usize {
    std::iter::successors(Some(input.to_string()), |i| Some(look_and_say(i)))
        .nth(50)
        .unwrap()
        .len()
}

fn look_and_say(input: &str) -> String {
    input.chars().chunk_by(|&c| c).into_iter().fold(
        String::with_capacity(input.len()),
        |mut output, (c, g)| {
            output.push_str(&g.count().to_string());
            output.push(c);
            output
        },
    )
}
