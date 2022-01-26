use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines().collect::<Vec<_>>();
    println!("count = {}", lines.len());

    let data = Data::new(&lines);

    println!("location count = {}", data.locations.len());
    println!("route count = {}", data.routes().len());

    let min_distance = data
        .routes()
        .iter()
        .min_by_key(|x| x.distance)
        .unwrap()
        .distance;

    println!("min. distance = {}", min_distance);

    let max_distance = data
        .routes()
        .iter()
        .max_by_key(|x| x.distance)
        .unwrap()
        .distance;

    println!("max. distance = {}", max_distance);
}

type Location<'a> = &'a str;
type LocationPair<'a> = (Location<'a>, Location<'a>);

struct Route<'a> {
    locations: Vec<Location<'a>>,
    distance: u32,
}

struct Data<'a> {
    locations: HashSet<Location<'a>>,
    links: HashMap<LocationPair<'a>, u32>,
}

impl<'a> Data<'a> {
    fn new(lines: &'a Vec<&str>) -> Data<'a> {
        let mut locations = HashSet::new();
        let mut links = HashMap::new();
        for line in lines {
            let tmp = Data::parse_line(line);
            locations.insert(tmp.0 .0);
            locations.insert(tmp.0 .1);
            // Insert links twice as they're bi-directional
            links.insert((tmp.0 .0, tmp.0 .1), tmp.1);
            links.insert((tmp.0 .1, tmp.0 .0), tmp.1);
        }
        Data { locations, links }
    }

    fn parse_line(line: &str) -> (LocationPair, u32) {
        let split = line.split_whitespace().collect::<Vec<_>>();
        let from = split[0];
        let to = split[2];
        let d = u32::from_str_radix(split[4], 10).unwrap();
        ((from, to), d)
    }

    fn routes(&self) -> Vec<Route<'a>> {
        let mut result = Vec::new();

        let permutations = self.locations.iter();
        let permutations = permutations.permutations(self.locations.len());
        let permutations = permutations.map(|x| x.iter().map(|y| **y).collect_vec());
        let permutations = permutations.collect_vec();

        for l in permutations {
            let mut d = 0;
            for i in 1..l.len() {
                let pair = (l[i - 1], l[i]);
                d += self.links.get(&pair).unwrap_or(&u32::MAX);
            }

            result.push(Route {
                locations: l,
                distance: d,
            });
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(
            Data::parse_line("London to Dublin = 464"),
            (("London", "Dublin"), 464)
        );
    }

    #[test]
    fn two() {
        let lines = vec![
            "London to Dublin = 464",
            "London to Belfast = 518",
            "Dublin to Belfast = 141",
        ];
        let d = Data::new(&lines);
        assert_eq!(d.links.len(), lines.len() * 2);
        assert_eq!(d.links.get(&("London", "Dublin")).unwrap(), &464);
        assert_eq!(d.links.get(&("Dublin", "London")).unwrap(), &464);
        assert_eq!(d.links.get(&("London", "Belfast")).unwrap(), &518);
        assert_eq!(d.links.get(&("Belfast", "London")).unwrap(), &518);
        assert_eq!(d.links.get(&("Dublin", "Belfast")).unwrap(), &141);
        assert_eq!(d.links.get(&("Belfast", "Dublin")).unwrap(), &141);
        println!("location count = {}", d.locations.len());

        for r in d.routes() {
            println!("{} {:?}", r.distance, r.locations);
        }

        let min_distance = d
            .routes()
            .iter()
            .min_by_key(|x| x.distance)
            .unwrap()
            .distance;
        assert_eq!(min_distance, 605);
    }
}
