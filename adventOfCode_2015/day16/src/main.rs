use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let sues = input.lines().map(Sue::from_line);

    let mut my_info = HashMap::new();
    my_info.insert("children", 3);
    my_info.insert("cats", 7);
    my_info.insert("samoyeds", 2);
    my_info.insert("pomeranians", 3);
    my_info.insert("akitas", 0);
    my_info.insert("vizslas", 0);
    my_info.insert("goldfish", 5);
    my_info.insert("trees", 3);
    my_info.insert("cars", 2);
    my_info.insert("perfumes", 1);

    let tmp = sues.clone().filter(|s| s.matches_info(&my_info));
    let tmp = tmp.collect::<Vec<_>>();
    println!("{tmp:?}");
    assert_eq!(tmp.first().unwrap().number, 213);

    // PART TWO
    println!("PART TWO");
    let tmp = sues.filter(|s| s.matches_info_parttwo(&my_info));
    let tmp = tmp.collect::<Vec<_>>();
    println!("{tmp:?}");
}

#[derive(Debug)]
struct Sue {
    number: i32,
    info: HashMap<String, i32>,
}

impl Sue {
    fn from_line(line: &str) -> Sue {
        let split = line
            .split(&[' ', ',', ':'])
            .filter(|x| x.is_empty() == false)
            .collect::<Vec<_>>();

        let number = split[1].parse::<i32>().unwrap();

        let mut info = HashMap::new();
        for i in (2..split.len()).step_by(2) {
            let k = split[i].to_string();
            let v = split[i + 1].parse::<i32>().unwrap();
            info.insert(k, v);
        }

        Sue { number, info }
    }

    fn matches_info(&self, info: &HashMap<&str, i32>) -> bool {
        for (k, v) in info {
            let tmp = self.info.get(*k);
            if tmp.is_some() && tmp.unwrap() != v {
                return false;
            }
        }
        true
    }

    fn matches_info_parttwo(&self, info: &HashMap<&str, i32>) -> bool {
        // cats and trees readings indicates that there are greater than that many
        // pomeranians and goldfish readings indicate that there are fewer than that many

        for (k, target_v) in info {
            match (k, self.info.get(*k)) {
                (&"cats", Some(actual_v)) | (&"trees", Some(actual_v)) => {
                    if actual_v <= target_v {
                        return false;
                    }
                }
                (&"pomeranians", Some(actual_v)) | (&"goldfish", Some(actual_v)) => {
                    if actual_v >= target_v {
                        return false;
                    }
                }
                (_, Some(actual_v)) => {
                    if actual_v != target_v {
                        return false;
                    }
                }
                _ => (),
            };

            // let tmp = self.info.get(*k);
            // if tmp.is_some() {
            //     if k == &"cats" || k == &"trees" {
            //         if tmp.unwrap() <= v {
            //             return false;
            //         }
            //     } else if k == &"pomeranians" || k == &"goldfish" {
            //         if tmp.unwrap() >= v {
            //             return false;
            //         }
            //     } else {
            //         if tmp.unwrap() != v {
            //             return false;
            //         }
            //     }
            // }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let mut my_info = HashMap::new();
        my_info.insert("children", 3);
        my_info.insert("cats", 7);
        my_info.insert("samoyeds", 2);

        let sue = Sue::from_line("Sue 11: goldfish: 1, perfumes: 4, cars: 6");
        assert_eq!(sue.matches_info(&my_info), true);

        let sue = Sue::from_line("Sue 11: goldfish: 1, perfumes: 4, cats: 7");
        assert_eq!(sue.matches_info(&my_info), true);

        let sue = Sue::from_line("Sue 11: goldfish: 1, perfumes: 4, cats: 3");
        assert_eq!(sue.matches_info(&my_info), false);
    }
}
