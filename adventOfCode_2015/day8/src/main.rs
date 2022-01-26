fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines().collect::<Vec<_>>();

    println!("count = {}", lines.len());

    let mut total_code_chars = 0usize;
    let mut total_mem_chars = 0usize;

    for line in &lines {
        let (code_chars, mem_chars) = lengths(line, transform);
        total_code_chars += code_chars;
        total_mem_chars += mem_chars;
    }

    println!(
        "{} - {} = {}",
        total_code_chars,
        total_mem_chars,
        total_code_chars - total_mem_chars
    );

    let mut total_code_chars = 0usize;
    let mut total_encode_chars = 0usize;

    for line in &lines {
        let (code_chars, encode_chars) = lengths(line, transform_two);
        total_code_chars += code_chars;
        total_encode_chars += encode_chars;
    }

    println!(
        "{} - {} = {}",
        total_encode_chars,
        total_code_chars,
        total_encode_chars - total_code_chars
    );
}

fn transform(input: &str) -> String {
    assert!(input.starts_with(r#"""#));
    assert!(input.ends_with(r#"""#));
    assert!(input.is_ascii());

    // This code assumes the input is correctly formed.

    let mut output = String::new();
    let mut iter = input.chars().peekable();
    iter.next(); // Skip start speechmarks
    iter.next_back(); // Skip last speechmarks

    while let Some(c) = iter.next() {
        if c == '\\' && iter.peek().unwrap() == &'\\' {
            iter.next();
            output.push('\\');
        } else if c == '\\' && iter.peek().unwrap() == &'\"' {
            iter.next();
            output.push('\"');
        } else if c == '\\' && iter.peek().unwrap() == &'x' {
            iter.next();
            let mut hex = String::new();
            hex.push(iter.next().unwrap());
            hex.push(iter.next().unwrap());
            let val = u32::from_str_radix(&hex, 16).unwrap();
            output.push(char::from_u32(val).unwrap());
        } else {
            output.push(c);
        }
    }

    output
}

fn transform_two(input: &str) -> String {
    assert!(input.starts_with(r#"""#));
    assert!(input.ends_with(r#"""#));
    assert!(input.is_ascii());

    // This code assumes the input is correctly formed.

    let mut output = String::new();
    let mut iter = input.chars().peekable();

    output.push('\"');

    while let Some(c) = iter.next() {
        if c == '\"' {
            output.push_str("\\\"");
        } else if c == '\\' {
            output.push_str("\\\\");
        } else {
            output.push(c);
        }
    }

    output.push('\"');
    output
}

fn lengths(input: &str, f: fn(&str) -> String) -> (usize, usize) {
    (input.chars().count(), f(input).chars().count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(lengths(r#""""#, transform), (2, 0));
        assert_eq!(lengths(r#""abc""#, transform), (5, 3));
        assert_eq!(lengths(r#""aaa\"aaa""#, transform), (10, 7));
        assert_eq!(lengths(r#""\x27""#, transform), (6, 1));
    }

    #[test]
    fn two() {
        assert_eq!(lengths(r#""\\x12""#, transform), (7, 4));
    }

    #[test]
    fn three() {
        assert_eq!(transform(r#""\x41""#), r#"A"#);
        assert_eq!(transform(r#""abc""#), r#"abc"#);
        assert_eq!(transform(r#""aaa\"aaa""#), r#"aaa"aaa"#);
        assert_eq!(transform(r#""\x27""#), r#"'"#);
        assert_eq!(transform(r#""\\x12""#), r#"\x12"#);
        assert_eq!(
            transform(r#""bycyxuafof\\\xa6uf\\axfozomj\\olh\x6a""#),
            r#"bycyxuafof\¦uf\axfozomj\olhj"#
        );
        assert_eq!(
            lengths(r#""bycyxuafof\\\xa6uf\\axfozomj\\olh\x6a""#, transform),
            (39, 28)
        );
    }

    #[test]
    fn four() {
        assert_eq!(transform_two(r#""""#), r#""\"\"""#);
        assert_eq!(lengths(r#""""#, transform_two), (2, 6));
        assert_eq!(transform_two(r#""abc""#), r#""\"abc\"""#);
        assert_eq!(lengths(r#""abc""#, transform_two), (5, 9));
        assert_eq!(transform_two(r#""aaa\"aaa""#), r#""\"aaa\\\"aaa\"""#);
        assert_eq!(lengths(r#""aaa\"aaa""#, transform_two), (10, 16));
        assert_eq!(transform_two(r#""\x27""#), r#""\"\\x27\"""#);
        assert_eq!(lengths(r#""\x27""#, transform_two), (6, 11));
    }
}
