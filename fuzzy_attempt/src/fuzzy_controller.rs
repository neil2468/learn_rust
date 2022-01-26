use super::fuzzy_set::FuzzySet;
use std::cmp;

const ASCIIGRAPH_DIMENSIONS: (u32, u32) = (100, 10);

pub struct FuzzyController {
    sets: Vec<FuzzySet>,
}

impl FuzzyController {
    pub fn new() -> FuzzyController {
        FuzzyController { sets: Vec::new() }
    }

    pub fn add_set(&mut self, set: FuzzySet) {
        self.sets.push(set);
    }

    pub fn membership_asciigraphs(&self) -> Vec<String> {
        let range_x_min = self.sets.iter().fold(i32::MAX, |acc, ele| cmp::min(acc, ele.range_x().0));
        let range_x_max = self.sets.iter().fold(i32::MIN, |acc, ele| cmp::max(acc, ele.range_x().1));
        self.sets
            .iter()
            .map(|s| s.membership_asciigraph(range_x_min..=range_x_max, ASCIIGRAPH_DIMENSIONS))
            .collect()
    }

    pub fn fuzzify(&self, value: i32) -> Vec<(&str, f64)> {
        self.sets
            .iter()
            .map(|s| (s.name(), s.membership(value)))
            .collect()        
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Test for panics
        let mut c = FuzzyController::new();
        c.add_set(FuzzySet::new("A", &[(0, 1.0), (20, 1.0), (40, 0.0)]));
        c.add_set(FuzzySet::new(
            "B",
            &[(30, 0.0), (60, 0.25), (70, 1.0), (90, 1.0), (100, 0.0)],
        ));
    
        for graph in c.membership_asciigraphs() {
            assert_ne!(graph.len(), 0);
        }
    }
}
