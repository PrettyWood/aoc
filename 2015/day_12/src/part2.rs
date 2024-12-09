use serde_json::{self, Value};

pub fn solve_part2(input: &str) -> isize {
    let value: Value = serde_json::from_str(input).expect("should be a valid json");
    get_sum(&value)
}

fn get_sum(value: &Value) -> isize {
    match value {
        Value::Number(v) => v.as_i64().expect("should be a i64") as isize,
        Value::Array(array) => array.iter().map(get_sum).sum(),
        Value::Object(obj) => {
            if obj
                .values()
                .any(|v| matches!(v, Value::String(s) if s == "red"))
            {
                return 0;
            }

            obj.values().map(get_sum).sum()
        }
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", 6)]
    #[case(r#"[1,{"c":"red","b":2},3]"#, 4)]
    #[case(r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0)]
    #[case(r#"[1,"red",5]"#, 6)]
    fn test_part_2(#[case] input: &str, #[case] expected: isize) {
        assert_eq!(solve_part2(input), expected);
    }
}
