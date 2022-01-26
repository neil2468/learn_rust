use std::collections::HashSet;

fn main() {
    let mut x = str_to_array("cqjxjnds");
    println!("old password {}", x.iter().collect::<String>());
    loop {
        add_one(&mut x);
        if is_ok(&x) {
            break;
        }
    }
    println!("new password {}", x.iter().collect::<String>());
    loop {
        add_one(&mut x);
        if is_ok(&x) {
            break;
        }
    }
    println!("new password {}", x.iter().collect::<String>());
}

fn is_ok(inp: &[char]) -> bool {
    inp.len() == 8
        && is_ok_twoplus_pairs(inp)
        && is_ok_increasing_straight(inp)
        && is_ok_invalid_chars(inp)
}

fn is_ok_twoplus_pairs(inp: &[char]) -> bool {
    let mut pair_letters: HashSet<&char> = HashSet::new();
    let mut pair_count = 0;
    let mut iter = inp.iter().peekable();
    while let Some(c) = iter.next() {
        if !pair_letters.contains(c) {
            if iter.peek() == Some(&c) {
                pair_count += 1;
                pair_letters.insert(&c);
                iter.next(); // Only count non-overlapping pairs
            }
        }
    }
    pair_count >= 2
}

fn is_ok_increasing_straight(inp: &[char]) -> bool {
    for i in 2..inp.len() {
        let v1 = inp[i - 2] as i32;
        let v2 = inp[i - 1] as i32;
        let v3 = inp[i] as i32;
        if v2 - v1 == 1 && v3 - v2 == 1 {
            return true;
        }
    }
    false
}

fn is_ok_invalid_chars(inp: &[char]) -> bool {
    !(inp.contains(&'i') || inp.contains(&'o') || inp.contains(&'l'))
}

fn add_one(inp: &mut [char]) {
    for i in (0..inp.len()).rev() {
        inp[i] = next_char(inp[i]);
        if inp[i] != 'a' {
            break;
        }
    }
}

fn next_char(inp: char) -> char {
    assert!(inp.is_ascii_alphabetic());
    match inp {
        'z' => 'a',
        _ => char::from_u32((inp as u32) + 1).unwrap(),
    }
}

fn str_to_array(inp: &str) -> [char; 8] {
    let mut out = ['a'; 8];
    let mut iter = inp.chars();
    for i in 0..8 {
        out[i] = iter.next().unwrap();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let foo = is_ok_increasing_straight;
        assert_eq!(foo(&str_to_array("aaaaaaaa")), false);
        assert_eq!(foo(&str_to_array("abdaaaaa")), false);
        assert_eq!(foo(&str_to_array("aaaabcaa")), true);
        assert_eq!(foo(&str_to_array("aaxyzaaaa")), true);
        assert_eq!(foo(&str_to_array("axyzaaaaa")), true);
        assert_eq!(foo(&str_to_array("axyzabcaa")), true);
    }

    #[test]
    fn two() {
        assert_eq!(is_ok_invalid_chars(&str_to_array("aaaabbbb")), true);
        assert_eq!(is_ok_invalid_chars(&str_to_array("aoaabbbb")), false);
        assert_eq!(is_ok_invalid_chars(&str_to_array("aaaoabbb")), false);
        assert_eq!(is_ok_invalid_chars(&str_to_array("aiaabbbb")), false);
        assert_eq!(is_ok_invalid_chars(&str_to_array("aaibslso")), false);
    }

    #[test]
    fn three() {
        assert_eq!(is_ok_twoplus_pairs(&str_to_array("abcdefgh")), false);
        assert_eq!(is_ok_twoplus_pairs(&str_to_array("aacdefgh")), false);
        assert_eq!(is_ok_twoplus_pairs(&str_to_array("abcdefaa")), false);
        assert_eq!(is_ok_twoplus_pairs(&str_to_array("aacdefaa")), false);
        assert_eq!(is_ok_twoplus_pairs(&str_to_array("aacdefvv")), true);
        assert_eq!(is_ok_twoplus_pairs(&str_to_array("abbceffg")), true);
    }

    #[test]
    fn four() {
        let first_test = is_ok_increasing_straight;
        let second_test = is_ok_invalid_chars;
        let third_test = is_ok_twoplus_pairs;

        // hijklmmn meets the first requirement (because it contains the straight hij)
        // but fails the second requirement requirement (because it contains i and l).
        assert_eq!(first_test(&str_to_array("hijklmmn")), true);
        assert_eq!(second_test(&str_to_array("hijklmmn")), false);

        // abbceffg meets the third requirement (because it repeats bb and ff)
        // but fails the first requirement.
        assert_eq!(third_test(&str_to_array("abbceffg")), true);
        assert_eq!(first_test(&str_to_array("abbceffg")), false);

        // abbcegjk fails the third requirement, because it only has one double letter (bb).
        assert_eq!(third_test(&str_to_array("abbcegjk")), false);

        // The next password after abcdefgh is abcdffaa.
        let mut s = str_to_array("abcdefgh");
        loop {
            add_one(&mut s);
            if is_ok(&s) {
                break;
            }
        }
        assert_eq!(s, str_to_array("abcdffaa"));
    }
}
