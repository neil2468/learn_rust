fn main() {
    let input = include_str!("input.txt");
    let containers = input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut solutions: Vec<Vec<i32>> = Vec::new();
    foo(&containers, 0, 150, &mut solutions, &Vec::new());
    println!("{}", solutions.len());
    assert_eq!(solutions.len(), 1304);

    println!("PART TWO...");

    let min = solutions.iter().map(|x| x.len()).min().unwrap();
    println!("Smallest solutions have {} containers", min);

    let count = solutions.iter().filter(|x| x.len() == min).count();
    println!("There are {} smallest solutions", count);
}

fn foo(
    containers: &Vec<i32>,
    offset: usize,
    eggnog: i32,
    solutions: &mut Vec<Vec<i32>>,
    current_solution: &Vec<i32>,
) {
    // This code inspired by...
    // https://github.com/alokmenghrajani/adventofcode/blob/master/src/year_2015/day17.rs

    if eggnog < 0 {
        return;
    }
    if offset == containers.len() {
        if eggnog == 0 {
            solutions.push(current_solution.clone());
        }
        return;
    }

    foo(containers, offset + 1, eggnog, solutions, current_solution);

    let mut current_solution = current_solution.clone();
    current_solution.push(containers[offset]);
    foo(
        containers,
        offset + 1,
        eggnog - containers[offset],
        solutions,
        &current_solution,
    );
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn one() {
        let containers = vec![20, 15, 10, 5, 5];

        // Exhaustive search
        let mut solutions = Vec::new();
        for k in 1..=containers.len() {
            for p in containers.iter().combinations(k) {
                let s: i32 = p.iter().copied().sum();
                if s == 25 {
                    solutions.push(p);
                }
            }
        }
        println!("{solutions:?}");
    }

    #[test]
    fn two() {
        let mut containers: Vec<i32> = vec![20, 15, 10, 5, 5];
        containers.sort();
        containers.reverse();
        let mut solutions: Vec<Vec<i32>> = Vec::new();
        let mut current_solution: Vec<i32> = Vec::new();

        foo(
            &containers,
            0,
            25,
            &mut solutions,
            &mut &mut current_solution,
        );

        println!("{solutions:?}");
    }
}
