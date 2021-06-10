use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

#[derive(Debug)]
struct Point {
    x: i32,
    y: f64,
}

#[derive(Debug)]
pub struct FuzzySet {
    name: String,
    points: Vec<Point>,
    x_min: i32,
    x_max: i32,
    slopes: Vec<f64>,
}

impl FuzzySet {
    pub fn new(name: &str, points: &[(i32, f64)]) -> FuzzySet {
        let mut tmp_points: Vec<Point> = points
            .iter()
            .map(|x| Point { x: x.0, y: x.1 })
            .collect::<Vec<_>>();
        tmp_points.sort_by(|a, b| a.x.cmp(&b.x));
        let x_min = tmp_points.first().unwrap().x;
        let x_max = tmp_points.last().unwrap().x;

        let slopes: Vec<f64> = (0..(tmp_points.len() - 1))
            .map(|i| -> f64 {
                ((tmp_points[i + 1].y - tmp_points[i].y) as f64)
                    / ((tmp_points[i + 1].x - tmp_points[i].x) as f64)
            })
            .collect();

        FuzzySet {
            name: name.to_string(),
            points: tmp_points,
            x_min: x_min,
            x_max: x_max,
            slopes: slopes,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn membership_asciigraph(
        &self,
        value_range: std::ops::RangeInclusive<i32>,
        dimensions: (u32, u32),
    ) -> String {
        let data = value_range
            .map(|x| (x as f64, self.membership(x)))
            .collect();
        let s1 = Plot::new(data).point_style(PointStyle::new().marker(PointMarker::Cross));
        let v = ContinuousView::new().add(s1);

        let mut result = String::new();
        result.push_str(&format!(
            "Membership function of set '{}'...\n",
            self.name()
        ));
        result.push_str(&format!(
            "  points = {}\n",
            self.points
                .iter()
                .map(|p| format!("({}, {})", p.x, p.y))
                .collect::<Vec<String>>()
                .join(", ")
        ));
        result.push_str(
            &Page::single(&v)
                .dimensions(dimensions.0, dimensions.1)
                .to_text()
                .unwrap(),
        );
        result
    }

    pub fn membership(&self, value: i32) -> f64 {
        let mut result = 0.0;
        if value >= self.x_min && value <= self.x_max {
            // Find nearest lower point
            let (lower_index, _) = self
                .points
                .iter()
                .enumerate()
                .filter(|e| e.1.x <= value)
                .last()
                .unwrap();

            // Calculate membership
            result = self.points[lower_index].y;
            if lower_index < self.slopes.len() {
                let lower_point = &self.points[lower_index];
                let slope = &self.slopes[lower_index];
                result += ((value - lower_point.x) as f64) * slope;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let set = FuzzySet::new(
            "A",
            &[(30, 0.0), (60, 0.50), (70, 1.0), (90, 1.0), (100, 0.0)],
        );

        // Test membership calculation
        assert_eq!(set.membership(65), 0.75);
    }

    #[test]
    fn test_2() {
        let set = FuzzySet::new(
            "A",
            &[(30, 0.0), (60, 0.50), (70, 1.0), (90, 1.0), (100, 0.0)],
        );

        // Test...
        // - we can calculate membership for within and outside the range of the set's points without panics
        // - values outside the range of the set's points have membership of 0.0
        for value in -10..110 {
            let m = set.membership(value);
            if value < 30 || value > 100 {
                assert_eq!(m, 0.0);
            }
        }
    }
}
