use std::cmp::max;

fn main() {
    let input = include_str!("input.txt");
    let ingredients = input.lines().map(Ingredient::from_line).collect::<Vec<_>>();
    let teaspoon_values = teaspoon_values(ingredients.len(), 100);
    println!("{}", teaspoon_values.len());

    let mut max_score = 0i32;
    for (i, tv) in teaspoon_values.iter().enumerate() {
        max_score = max(max_score, recipe_score(ingredients.iter(), tv.iter()));
    }
    println!("{max_score}");
    assert_eq!(max_score, 21367368);

    // PART TWO
    println!();
    println!("PART TWO...");
    let mut max_score = 0i32;
    for (i, tv) in teaspoon_values.iter().enumerate() {
        if recipe_calories(ingredients.iter(), tv.iter()) == 500 {
            max_score = max(max_score, recipe_score(ingredients.iter(), tv.iter()));
        }
    }
    println!("{max_score}");
    assert_eq!(max_score, 1766400);
}

struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn get_field(&self, name: &str) -> Result<i32, &str> {
        match name {
            "capacity" => Ok(self.capacity),
            "durability" => Ok(self.durability),
            "flavor" => Ok(self.flavor),
            "texture" => Ok(self.texture),
            "calories" => Ok(self.calories),
            _ => Err("Unknown field"),
        }
    }

    fn from_line(line: &str) -> Ingredient {
        let split = line
            .split(&[' ', ',', ':'])
            .filter(|x| x.is_empty() == false)
            .collect::<Vec<_>>();
        debug_assert_eq!(split.len(), 11);

        let name = split[0].to_string();
        let capacity = split[2].parse::<i32>().unwrap();
        let durability = split[4].parse::<i32>().unwrap();
        let flavor = split[6].parse::<i32>().unwrap();
        let texture = split[8].parse::<i32>().unwrap();
        let calories = split[10].parse::<i32>().unwrap();

        Ingredient {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

fn recipe_score<'a, I, T>(ingredients: I, teaspoons: T) -> i32
where
    I: Iterator<Item = &'a Ingredient> + Clone,
    T: Iterator<Item = &'a i32> + Clone,
{
    let calc_total = |field: &str| -> i32 {
        ingredients
            .clone()
            .map(|x| x.get_field(field).unwrap())
            .zip(teaspoons.clone())
            .map(|(a, b)| a * b)
            .sum()
    };

    let totals = vec![
        calc_total("capacity"),
        calc_total("durability"),
        calc_total("flavor"),
        calc_total("texture"),
    ];

    totals.iter().map(|&x| max(0, x)).product()
}

fn recipe_calories<'a, I, T>(ingredients: I, teaspoons: T) -> i32
where
    I: Iterator<Item = &'a Ingredient> + Clone,
    T: Iterator<Item = &'a i32> + Clone,
{
    ingredients
        .clone()
        .map(|x| x.calories)
        .zip(teaspoons.clone())
        .map(|(a, b)| a * b)
        .sum()
}

fn teaspoon_values(width: usize, sum: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    if width == 1 {
        let mut tmp = Vec::new();
        tmp.push(sum);
        result.push(tmp);
    } else {
        for this in 0..=sum {
            for mut ff in teaspoon_values(width - 1, sum - this) {
                let mut tmp = Vec::new();
                tmp.push(this);
                tmp.append(&mut ff);
                result.push(tmp);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let i = Ingredient::from_line(
            "Sprinkles: capacity 2, durability 0, flavor -2, texture 0, calories 3",
        );
        assert_eq!(i.name, "Sprinkles");
        assert_eq!(i.capacity, 2);
        assert_eq!(i.durability, 0);
        assert_eq!(i.flavor, -2);
        assert_eq!(i.texture, 0);
        assert_eq!(i.calories, 3);
    }

    #[test]
    fn two() {
        let ingredients = vec![
            Ingredient::from_line(
                "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
            ),
            Ingredient::from_line(
                "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
            ),
        ];

        let x = recipe_score(ingredients.iter(), [44, 56].iter());
        assert_eq!(x, 62842880);
    }

    #[test]
    fn three() {
        let x = teaspoon_values(1, 3);
        assert_eq!(x, [[3]]);

        let x = teaspoon_values(2, 3);
        assert_eq!(x, [[0, 3], [1, 2], [2, 1], [3, 0]]);

        let x = teaspoon_values(3, 3);
        assert_eq!(
            x,
            [
                [0, 0, 3],
                [0, 1, 2],
                [0, 2, 1],
                [0, 3, 0],
                [1, 0, 2],
                [1, 1, 1],
                [1, 2, 0],
                [2, 0, 1],
                [2, 1, 0],
                [3, 0, 0]
            ]
        );
    }
}
