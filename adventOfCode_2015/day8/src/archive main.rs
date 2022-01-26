fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines().collect::<Vec<_>>();

    println!("count = {}", lines.len());

    let mut i = 1usize;
    let mut total_code_chars = 0usize;
    let mut total_mem_chars = 0usize;

    for line in lines {
        let (code_chars, mem_chars) = lengths(line);
        total_code_chars += code_chars;
        total_mem_chars += mem_chars;
        println!(
            "{:?}",
            (
                i,
                line,
                code_chars,
                mem_chars,
                total_code_chars,
                total_mem_chars
            )
        );
        i += 1;
    }

    println!(
        "{} - {} = {}",
        total_code_chars,
        total_mem_chars,
        total_code_chars - total_mem_chars
    );
}

fn transform_len(input: &str) -> usize {
    assert!(input.starts_with(r#"""#));
    assert!(input.ends_with(r#"""#));
    assert!(input.is_ascii());

    // This code assumes the input is correctly formed.

    // let a = input.match_indices(r#"\\"#).count();
    // let b = input.match_indices(r#"\""#).count();
    // let c = input.match_indices(r#"\x"#).count();
    // input.len() - (2 + a + b + (c * 3))

    let mut tmp = input.to_string();
    let a = tmp.match_indices(r#"\\"#).count();
    tmp = tmp.replace(r#"\\"#, "");
    let b = tmp.match_indices(r#"\""#).count();
    tmp = tmp.replace(r#"\""#, "");
    let c = tmp.match_indices(r#"\x"#).count();
    input.len() - (2 + a + b + (c * 3))
}

// fn transform(input: &str) -> String {
//     assert!(input.starts_with("\""));
//     assert!(input.ends_with("\""));
//     println!("input = {}", input);

//     // This code assumes all lines are valid. Invalid lines may not be processed correctly.

//     let mut a = input.chars().peekable();

//     a.next(); // Skip start speechmarks
//     a.next_back(); // Skip last speechmarks

//     let mut output = String::new();
//     while let Some(c) = a.next() {
//         println!("c = {}", c);
//         if c == '\\' && a.peek() == Some(&'\\') {
//             output.push('\\');
//             a.next();
//         } else if c == '\\' && a.peek() == Some(&'\"') {
//             output.push('\"');
//             a.next();
//         } else if c == '\\' && a.peek() == Some(&'x') {
//             a.next();

//             let mut hex = String::new();
//             hex.push(a.next().unwrap());
//             hex.push(a.next().unwrap());
//             let val = u32::from_str_radix(&hex, 16).unwrap();
//             let cc = char::from_u32(val).unwrap();
//             output.push(char::from_u32(val).unwrap());
//             println!("hex: {:0x} {}", val, cc);
//         } else {
//             output.push(c);
//         }
//     }

//     output
// }

fn lengths(input: &str) -> (usize, usize) {
    (input.len(), transform_len(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(lengths(r#""""#), (2, 0));
        assert_eq!(lengths(r#""abc""#), (5, 3));
        assert_eq!(lengths(r#""aaa\"aaa""#), (10, 7));
        assert_eq!(lengths(r#""\x27""#), (6, 1));
    }

    #[test]
    fn two() {
        assert_eq!(lengths(r#""\\x12""#), (7, 4));
    }
}
