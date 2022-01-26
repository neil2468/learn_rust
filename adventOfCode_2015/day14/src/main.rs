use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("input.txt");
    let herd = input.lines().map(Reindeer::from_line).collect::<Vec<_>>();

    let distances_flown = herd.iter().map(|x| x.distance_flown(2503));

    let max_distance = distances_flown.max().unwrap();
    println!("max distance {}", max_distance);
    assert_eq!(max_distance, 2660);

    for r in &herd {
        println!("{} {}", r.name, r.distance_flown(2503));
    }

    println!();
    println!("Part Two...");
    let score_table = Reindeer::score_race(&herd, 2503);
    let winning_score = score_table.iter().map(|r| r.1).max().unwrap();
    println!("Winning score: {}", winning_score);
    assert_eq!(winning_score, &1256);
}

struct Reindeer {
    name: String,
    speed: i32,     // km per second
    fly_time: i32,  // seconds
    rest_time: i32, // seconds
}

impl Reindeer {
    fn from_line(line: &str) -> Reindeer {
        let split = line.split_whitespace().collect::<Vec<_>>();
        debug_assert_eq!(split.len(), 15);

        let name = String::from_str(split[0]).unwrap();
        let speed = split[3].parse::<i32>().unwrap();
        let fly_time = split[6].parse::<i32>().unwrap();
        let rest_time = split[13].parse::<i32>().unwrap();

        Reindeer {
            name,
            speed,
            fly_time,
            rest_time,
        }
    }

    fn distance_flown(&self, seconds: i32) -> i32 {
        let cycle_time = self.fly_time + self.rest_time;
        let cycles = seconds / cycle_time;
        let remain_time = seconds - (cycles * cycle_time);
        let fly_time = (cycles * self.fly_time) + i32::min(remain_time, self.fly_time);
        fly_time * self.speed
    }

    fn score_race(herd: &Vec<Reindeer>, seconds: i32) -> HashMap<&String, i32> {
        let mut score_table = HashMap::new();
        for reindeer in herd.iter() {
            score_table.insert(&reindeer.name, 0);
        }

        for s in 1..=seconds {
            let mut scores = herd
                .iter()
                .map(|r| (&r.name, r.distance_flown(s)))
                .collect::<Vec<_>>();

            scores.sort_by(|a, b| b.1.cmp(&a.1));
            let max_score = scores.first().unwrap().1;
            let winners = scores
                .iter()
                .map_while(|r| if r.1 == max_score { Some(r.0) } else { None })
                .collect::<Vec<_>>();

            for name in &winners {
                *score_table.get_mut(name).unwrap() += 1;
            }
        }

        score_table
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn one() {
        let comet = Reindeer {
            name: "Comet".to_string(),
            speed: 14,
            fly_time: 10,
            rest_time: 127,
        };
        let dancer = Reindeer {
            name: "Dancer".to_string(),
            speed: 16,
            fly_time: 11,
            rest_time: 162,
        };
        assert_eq!(comet.distance_flown(1000), 1120);
        assert_eq!(dancer.distance_flown(1000), 1056);
    }

    #[test]
    fn two() {
        let tmp = Reindeer::from_line(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
        );
        assert_eq!(tmp.name, "Comet");
        assert_eq!(tmp.speed, 14);
        assert_eq!(tmp.fly_time, 10);
        assert_eq!(tmp.rest_time, 127);
        let tmp = Reindeer::from_line(
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        );
        assert_eq!(tmp.name, "Dancer");
        assert_eq!(tmp.speed, 16);
        assert_eq!(tmp.fly_time, 11);
        assert_eq!(tmp.rest_time, 162);
    }

    #[test]
    fn three() {
        let mut herd = Vec::new();
        herd.push(Reindeer::from_line(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
        ));
        herd.push(Reindeer::from_line(
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        ));
        herd.push(Reindeer::from_line(
            "Dancer2 can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        ));

        let score_table = Reindeer::score_race(&herd, 142);
        println!("{:?}", score_table);

        assert_eq!(score_table.get(&"Dancer".to_string()), Some(&139));
        assert_eq!(score_table.get(&"Dancer2".to_string()), Some(&139));
        assert_eq!(score_table.get(&"Comet".to_string()), Some(&3));
    }
}
