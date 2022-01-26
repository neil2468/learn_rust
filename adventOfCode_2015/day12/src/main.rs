use serde_json::Value;

fn main() {
    let input = include_str!("input.txt");
    let value: Value = serde_json::from_str(input).unwrap();

    println!("sum_part1: {}", sum_part1(&value));
    println!("sum_part2: {}", sum_part2(&value));
    assert_eq!(sum_part1(&value), 156366);
    assert_eq!(sum_part2(&value), 96852);
}

fn sum_part1(value: &Value) -> i64 {
    match value {
        Value::Array(vec) => vec.iter().map(sum_part1).sum::<i64>(),
        Value::Object(map) => map.values().map(sum_part1).sum::<i64>(),
        Value::Number(n) => n.as_i64().unwrap(),
        _ => 0,
    }
}

fn sum_part2(value: &Value) -> i64 {
    match value {
        Value::Array(vec) => vec.iter().map(sum_part2).sum::<i64>(),
        Value::Object(map) => {
            let have_red = map.values().any(|x| x.as_str() == Some("red"));
            if !have_red {
                map.values().map(sum_part2).sum::<i64>()
            } else {
                0
            }
        }
        Value::Number(n) => n.as_i64().unwrap(),
        _ => 0,
    }
}
