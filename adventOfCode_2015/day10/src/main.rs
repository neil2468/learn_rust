fn main() {
    assert_eq!(go("1"), "11");
    assert_eq!(go("11"), "21"); // 2 copies of digit 1
    assert_eq!(go("21"), "1211"); // one 2 followed by one 1
    assert_eq!(go("1211"), "111221"); // one 1, one 2, and two 1s
    assert_eq!(go("111221"), "312211"); // three 1s, two 2s, and one 1

    let mut s = "1321131112".to_string();
    for _ in 0..40 {
        s = go(&s);
    }
    println!("length {}", s.len());

    let mut s = "1321131112".to_string();
    for _ in 0..50 {
        s = go(&s);
    }
    println!("length {}", s.len());
}

fn go(inp: &str) -> String {
    assert!(inp.is_ascii());
    let mut out = String::new();
    let mut iter = inp.chars().peekable();
    let mut count = 0;
    while let Some(c) = iter.next() {
        count += 1;
        if iter.peek() != Some(&c) {
            out.push_str(format!("{}{}", count, c).as_str());
            count = 0;
        }
    }
    out
}
