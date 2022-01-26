use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let mut rules = Rules::from_lines(input.lines());

    // PART ONE
    let p = rules.people.iter();
    let p = p.permutations(rules.people.len());
    let p = p.map(|x| x.iter().map(|y| **y).collect_vec());
    let p = p.map(|x| (x.clone(), rules.total_gain(&x)));
    let mut p = p.collect_vec();
    p.sort_by(|a, b| a.1.cmp(&b.1));
    println!("{:?}", p);
    let max_gain = p.last().unwrap().1;
    println!("max gain: {}", max_gain);
    assert_eq!(max_gain, 618);

    // PART TWO
    for person in &rules.people {
        rules.data.insert(("Me", person), 0);
        rules.data.insert((person, "Me"), 0);
    }
    rules.people.insert("Me");
    let p = rules.people.iter();
    let p = p.permutations(rules.people.len());
    let p = p.map(|x| x.iter().map(|y| **y).collect_vec());
    let p = p.map(|x| (x.clone(), rules.total_gain(&x)));
    let mut p = p.collect_vec();
    p.sort_by(|a, b| a.1.cmp(&b.1));
    println!("{:?}", p);
    let max_gain = p.last().unwrap().1;
    println!("max gain: {}", max_gain);
    assert_eq!(max_gain, 601);
}

#[derive(Debug)]
struct Rules {
    data: HashMap<(&'static str, &'static str), i32>, // (person, neighbour) -> gain
    people: HashSet<&'static str>,
}

impl Rules {
    fn from_lines<I>(lines: I) -> Self
    where
        I: IntoIterator<Item = &'static str>,
    {
        let mut data = HashMap::new();
        let mut people = HashSet::new();
        for line in lines {
            let (k, v) = Rules::from_line(line);
            data.insert(k, v);
            people.insert(k.0);
        }
        Self { data, people }
    }

    fn from_line(line: &str) -> ((&str, &str), i32) {
        let line = &line[0..line.len() - 1];
        let split = line.split_whitespace().collect::<Vec<_>>();
        assert_eq!(split.len(), 11);
        let name_a = split[0];
        let name_b = split[10];
        let mut gain = split[3].parse::<i32>().unwrap();
        if split[2] == "lose" {
            gain *= -1;
        }
        ((name_a, name_b), gain)
    }

    fn total_gain(&self, seating: &Vec<&str>) -> i32 {
        assert!(seating.len() > 1);
        let mut gain = 0;
        for i in 1..seating.len() {
            let person = seating[i - 1];
            let neighbour = seating[i];
            gain += self.data.get(&(person, neighbour)).unwrap();
            gain += self.data.get(&(neighbour, person)).unwrap();
        }
        let person = seating.first().unwrap();
        let neighbour = seating.last().unwrap();
        gain += self.data.get(&(person, neighbour)).unwrap();
        gain += self.data.get(&(neighbour, person)).unwrap();
        gain
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn one() {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol.";

        let rules = Rules::from_lines(input.lines());
        assert_eq!(rules.data.len(), 12);
        assert_eq!(rules.data.get(&("Alice", "David")), Some(&-2));
        assert_eq!(rules.data.get(&("Carol", "Bob")), Some(&60));

        let mut people = rules.people.iter().collect::<Vec<_>>();
        people.sort();
        people.reverse();
        assert_eq!(people, vec![&"David", &"Carol", &"Bob", &"Alice"]);

        let seating = vec!["Alice", "Bob", "Carol", "David"];
        assert_eq!(rules.total_gain(&seating), 330);

        let p = seating.iter();
        let p = p.permutations(seating.len());
        let p = p.map(|x| x.iter().map(|y| **y).collect_vec());
        let p = p.map(|x| (x.clone(), rules.total_gain(&x)));
        let mut p = p.collect_vec();
        p.sort_by(|a, b| a.1.cmp(&b.1));
        println!("{:?}", p);

        let max_gain = p.last().unwrap().1;
        assert_eq!(max_gain, 330);
    }
}
