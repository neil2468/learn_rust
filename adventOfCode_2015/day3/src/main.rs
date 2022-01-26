use std::collections::HashSet;

fn main() {
    println!("Hello, world!");

    let input = include_str!("input.txt");

    println!(
        "houses_visited_at_least_once = {}",
        houses_visited_at_least_once(input)
    );

    println!(
        "houses_visited_at_least_once_part_two = {}",
        houses_visited_at_least_once_part_two(input)
    );
}

fn houses_visited_at_least_once(input: &str) -> usize {
    let mut position = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(position);

    for c in input.chars() {
        println!("{}", c);

        match c {
            '^' => position.1 += 1,
            '>' => position.0 += 1,
            'v' => position.1 -= 1,
            '<' => position.0 -= 1,
            _ => panic!(),
        }

        println!("{:?}", position);
        visited.insert(position);
    }

    println!("visited: {:?}", visited);

    visited.len()
}

fn houses_visited_at_least_once_part_two(input: &str) -> usize {
    let mut positions = vec![(0, 0), (0, 0)];
    let mut visited = HashSet::new();
    visited.insert(positions[0]);
    let mut index: usize = 0;

    for c in input.chars() {
        positions[index] = new_position(&c, &positions[index]);
        visited.insert(positions[index]);
        index = (index + 1) % positions.len();
    }

    println!("visited: {:?}", visited);

    visited.len()
}

fn new_position(c: &char, position: &(i32, i32)) -> (i32, i32) {
    let mut p = position.clone();
    match c {
        '^' => p.1 += 1,
        '>' => p.0 += 1,
        'v' => p.1 -= 1,
        '<' => p.0 -= 1,
        _ => panic!(),
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(houses_visited_at_least_once(&">"), 2);
        assert_eq!(houses_visited_at_least_once(&"^>v<"), 4);
        assert_eq!(houses_visited_at_least_once(&"^v^v^v^v^v"), 2);
    }

    #[test]
    fn two() {
        assert_eq!(houses_visited_at_least_once_part_two(&"^v"), 3);
        assert_eq!(houses_visited_at_least_once_part_two(&"^>v<"), 3);
        assert_eq!(houses_visited_at_least_once_part_two(&"^v^v^v^v^v"), 11);
    }
}
