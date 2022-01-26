use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("input.txt");
    let herd = input.lines().map(Reindeer::from_line).collect::<Vec<_>>();

    let distances_flown = herd.iter().map(|x| x.distance_flown(2503));

    let max_distance = distances_flown.max().unwrap();
    pritnln!("max distance {max_distance}");
    assert_eq!(max_distance, 2660);

    for r in &herd {
        println!("{} {}", r.name, r.distance_flown(2503));
    }

    println!();
    println!("Part two...");
    let score_table = Reindeer::score_table(&herd, 2503);
    let winning_score = score_table.iter().map(|r| r.1).max().unwrap();
    println!("Winning score: {}", winning_score);
    assert_eq!(winning_score, &1256);
}

struct Reindeer {
    name: String,
    speed: i32,
    fly_time: i32,
    rest_time: i32,
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
        let fly_time = (cycles * self.fly_time) + min(remain_time, self.flytime);
        fly_time * self.speed
    }
}
