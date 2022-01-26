use std::str::Lines;

fn main() {
    println!("Hello, world!");

    let input = include_str!("input.txt").lines();
    let mut grid = Grid::from_lines(input);
    println!("Dimensions: {}w {}h", grid.width(), grid.height());
    println!("Start on count: {}", grid.on_count());
    for _ in 0..100 {
        grid.advance_step();
    }
    println!("Finish on count: {}", grid.on_count());
}

struct Grid {
    step_number: i32,
    lights: Vec<Vec<bool>>,
}

impl Grid {
    fn from_lines(lines: Lines) -> Grid {
        let mut lights: Vec<Vec<bool>> = Vec::new();

        for line in lines {
            let tmp = line.chars().map(|c| c == '#').collect::<Vec<_>>();
            lights.push(tmp);
        }

        Grid {
            step_number: 0,
            lights,
        }
    }

    fn to_string(&self) -> String {
        let mut result = Vec::new();
        for y in 0..self.height() {
            result.push(
                self.lights[y]
                    .iter()
                    .map(|&x| if x { '#' } else { '.' })
                    .collect::<String>(),
            );
        }
        result.join("\n")
    }

    fn advance_step(&mut self) {
        let mut new_lights = self.lights.clone();

        for y in 0..self.height() {
            for x in 0..self.width() {
                // A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
                // A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
                if self.lights[y][x] {
                    new_lights[y][x] = [2, 3].contains(&(self.lights_surround_on_count(x, y)));
                } else {
                    new_lights[y][x] = self.lights_surround_on_count(x, y) == 3;
                }
            }
        }

        self.step_number += 1;
        self.lights = new_lights;
    }

    // Private method
    fn lights_surround_on_count(&self, x: usize, y: usize) -> usize {
        let iter_x = (x as i32) - 1..=(x as i32) + 1;
        let iter_y = (y as i32) - 1..=(y as i32) + 1;

        let max_x = (self.width() - 1) as i32;
        let max_y = (self.height() - 1) as i32;
        let tmp = iter_x
            .map(|x| iter_y.clone().map(move |y| (x, y)))
            .flatten()
            .filter(|(xx, yy)| {
                xx >= &0
                    && xx <= &max_x
                    && yy >= &0
                    && yy <= &max_y
                    && (xx, yy) != (&(x as i32), &(y as i32))
            });

        tmp.filter(|(xx, yy)| self.lights[(*yy as usize)][(*xx as usize)])
            .count()
    }

    fn width(&self) -> usize {
        self.lights.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.lights.len()
    }

    fn on_count(&self) -> usize {
        self.lights
            .iter()
            .map(|row| row.iter().filter(|&&x| x).count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        // .#.#.#
        // ...##.
        // #....#
        // ..#...
        // #.#..#
        // ####..
        let input = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..";
        let mut grid = Grid::from_lines(input.lines());
        assert_eq!(grid.to_string(), input);
        assert_eq!(grid.width(), 6);
        assert_eq!(grid.height(), 6);
        assert_eq!(grid.on_count(), 15);
        assert_eq!(grid.lights_surround_on_count(0, 0), 1);
        assert_eq!(grid.lights_surround_on_count(1, 1), 2);
        assert_eq!(grid.lights_surround_on_count(2, 5), 3);

        grid.advance_step();
        assert_eq!(grid.step_number, 1);
        assert_eq!(
            grid.to_string(),
            "..##..\n..##.#\n...##.\n......\n#.....\n#.##.."
        );

        grid.advance_step();
        assert_eq!(grid.step_number, 2);
        assert_eq!(
            grid.to_string(),
            "..###.\n......\n..###.\n......\n.#....\n.#...."
        );

        grid.advance_step();
        assert_eq!(grid.step_number, 3);
        assert_eq!(
            grid.to_string(),
            "...#..\n......\n...#..\n..##..\n......\n......"
        );

        grid.advance_step();
        assert_eq!(grid.step_number, 4);
        assert_eq!(
            grid.to_string(),
            "......\n......\n..##..\n..##..\n......\n......"
        );
        assert_eq!(grid.on_count(), 4);
    }
}
