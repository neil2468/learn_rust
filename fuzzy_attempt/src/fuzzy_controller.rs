use super::fuzzy_set::FuzzySet;

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

    pub fn membership_asciigraphs(&self, value_range: std::ops::RangeInclusive<i32>) -> Vec<String> {
        self.sets
            .iter()
            .map(|s| s.membership_asciigraph(value_range.clone(), ASCIIGRAPH_DIMENSIONS))
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
    
        for graph in c.membership_asciigraphs(0..=100) {
            assert_ne!(graph.len(), 0);
        }
    }
}
