use std::{collections::HashSet, iter::FromIterator};

fn main() {
    let input = include_str!("input.txt");
    println!("count = {}", input.lines().count());
    let count = input.lines().filter(|x| is_nice(x)).count();
    println!("nice count = {}", count);
    assert_eq!(count, 238);
    let count = input.lines().filter(|x| is_nice_part2(x)).count();
    println!("nice_part2 count = {}", count);
    assert_eq!(count, 69);
}

fn is_nice(line: &str) -> bool {
    // It does not contain the strings ab, cd, pq, or xy,
    // even if they are part of one of the other requirements.
    if line.contains(&"ab") || line.contains(&"cd") || line.contains(&"pq") || line.contains(&"xy")
    {
        return false;
    }

    // It contains at least one letter that appears twice in a row,
    // like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
    let mut has_repeated_letter = false;
    let chars: Vec<char> = line.chars().collect();
    for i in 0..chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            has_repeated_letter = true;
            break;
        }
    }
    if !has_repeated_letter {
        return false;
    }

    // It contains at least three vowels (aeiou only),
    // like aei, xazegov, or aeiouaeiouaeiou.

    // Take 1
    // let vowels = HashSet::<_>::from_iter(IntoIter::new(['a', 'e', 'i', 'o', 'u']));
    // let mut vowel_count = 0;
    // for c in chars {
    //     if vowels.contains(&c) {
    //         vowel_count += 1;
    //     }
    // }
    // if vowel_count < 3 {
    //     return false;
    // }

    // Take 2
    let vowels = HashSet::<_>::from_iter("aeiou".chars());
    let vowel_count = chars.iter().filter(|x| vowels.contains(x)).count();
    if vowel_count < 3 {
        return false;
    }

    true
}

fn is_nice_part2(line: &str) -> bool {
    is_nice_part2_rule1(line) && is_nice_part2_rule2(line)
}

fn is_nice_part2_rule1(line: &str) -> bool {
    let chars: Vec<_> = line.chars().collect();
    for i in 0..chars.len() - 3 {
        let a: String = chars[i..i + 2].iter().collect();
        let b: String = chars[i + 2..].iter().collect();
        let c = b.contains(&a);
        if c {
            return true;
        }
    }
    false
}

fn is_nice_part2_rule2(line: &str) -> bool {
    // for i in 0..line.len() - 2 {
    //     if &line[i..i] == &line[i + 2] {
    //         return true;
    //     }
    // }
    let chars: Vec<_> = line.chars().collect();
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice("aaa"), true);
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn two() {
        assert_eq!(is_nice("xy"), false);
        assert_eq!(is_nice("aa"), false);
    }

    #[test]
    fn three() {
        assert_eq!(is_nice_part2("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_part2("xxyxx"), true);
        assert_eq!(is_nice_part2("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_part2("ieodomkazucvgmuy"), false);
    }

    #[test]
    fn four() {
        assert_eq!(is_nice_part2_rule1("dyéxyxy"), true);
        assert_eq!(is_nice_part2_rule1("yéxyxy"), true);
        assert_eq!(is_nice_part2_rule1("xyxy"), true);
        assert_eq!(is_nice_part2_rule1("aabcdefgaa"), true);
        assert_eq!(is_nice_part2_rule1("abbaacdefgaa"), true);
        assert_eq!(is_nice_part2_rule1("aaa"), false);
        assert_eq!(is_nice_part2_rule1("aaaa"), true);
        assert_eq!(is_nice_part2_rule1("axybxy"), true);
    }

    #[test]
    fn five() {
        assert_eq!(is_nice_part2_rule2("xyx"), true);
        assert_eq!(is_nice_part2_rule2("abcdefeghi"), true);
        assert_eq!(is_nice_part2_rule2("aaa"), true);
    }
}
