fn main() {
    let mut grid = [[0u32; 1000]; 1000];

    let input = include_str!("input.txt");

    for line in input.lines() {
        let line = line.replace("turn ", "");
        let line: Vec<_> = line.split(|c| c == ' ' || c == ',').collect();
        let start = (
            line[1].parse::<usize>().unwrap(),
            line[2].parse::<usize>().unwrap(),
        );
        let end = (
            line[4].parse::<usize>().unwrap(),
            line[5].parse::<usize>().unwrap(),
        );

        println!("{:?}", line);

        for y in start.1..=end.1 {
            for x in start.0..=end.0 {
                // match line[0] {
                //     "on" => grid[x][y] = 1,
                //     "off" => grid[x][y] = 0,
                //     "toggle" => grid[x][y] = (grid[x][y] + 1) % 2,
                //     _ => panic!("Unknown command"),
                // }
                match line[0] {
                    "on" => grid[x][y] += 1,
                    "off" => grid[x][y] = if grid[x][y] > 0 { grid[x][y] - 1 } else { 0 },
                    "toggle" => grid[x][y] += 2,
                    _ => panic!("Unknown command"),
                }
            }
        }
    }

    // let tmp = grid.iter().flatten().filter(|x| **x == 1).count();
    // println!("{:?}", tmp);

    println!("{:?}", grid.iter().flatten().collect::<Vec<_>>());

    let tmp: u32 = grid.iter().flatten().sum();
    println!("{:?}", tmp);
}

// fn main() {
//     let input = include_str!("input.txt");
//     println!("count = {}", input.lines().count());

//     let mut grid = Grid::new();

//     for line in input.lines() {
//         let line: Vec<_> = line.split(|c| c == ' ' || c == ',').collect();
//         println!("{:?}", line);

//         if line[0] == "toggle" {
//             let start = (
//                 line[1].parse::<u32>().unwrap(),
//                 line[2].parse::<u32>().unwrap(),
//             );
//             let end = (
//                 line[4].parse::<u32>().unwrap(),
//                 line[5].parse::<u32>().unwrap(),
//             );
//             grid.toggle(start, end);
//         } else if line[0] == "turn" && line[1] == "on" {
//             let start = (
//                 line[2].parse::<u32>().unwrap(),
//                 line[3].parse::<u32>().unwrap(),
//             );
//             let end = (
//                 line[5].parse::<u32>().unwrap(),
//                 line[6].parse::<u32>().unwrap(),
//             );
//             grid.turn_on(start, end);
//         } else if line[0] == "turn" && line[1] == "off" {
//             let start = (
//                 line[2].parse::<u32>().unwrap(),
//                 line[3].parse::<u32>().unwrap(),
//             );
//             let end = (
//                 line[5].parse::<u32>().unwrap(),
//                 line[6].parse::<u32>().unwrap(),
//             );
//             grid.turn_off(start, end);
//         } else {
//             panic!("Unknown start of line '{:?}'", line)
//         }
//     }
//     println!("lit count: {}", grid.data.len());
// }

// type Point = (u32, u32);

// struct Grid {
//     x_max: u32,
//     y_max: u32,
//     data: HashSet<Point>,
// }

// impl Grid {
//     fn new() -> Grid {
//         Grid {
//             x_max: 9999,
//             y_max: 9999,
//             data: HashSet::new(),
//         }
//     }

//     fn display(&self) {
//         let path = std::path::Path::new("output.txt");
//         let mut file = std::fs::File::create(&path).unwrap();
//         for y in 0..=self.y_max {
//             let mut line = String::new();
//             for x in 0..=self.x_max {
//                 if self.data.contains(&(x, y)) {
//                     line.push('*');
//                 } else {
//                     line.push('_');
//                 }
//             }
//             line.push('\r');
//             line.push('\n');
//             file.write_all(line.as_bytes()).unwrap();
//         }
//     }

//     fn turn_on(&mut self, start: Point, end: Point) {
//         for y in start.1..=end.1 {
//             for x in start.0..=end.0 {
//                 self.data.insert((x, y));
//             }
//         }
//     }

//     fn turn_off(&mut self, start: Point, end: Point) {
//         for y in start.1..=end.1 {
//             for x in start.0..=end.0 {
//                 self.data.remove(&(x, y));
//             }
//         }
//     }

//     fn toggle(&mut self, start: Point, end: Point) {
//         for y in start.1..=end.1 {
//             for x in start.0..=end.0 {
//                 if self.data.contains(&(x, y)) {
//                     self.data.remove(&(x, y));
//                 } else {
//                     self.data.insert((x, y));
//                 }
//             }
//         }
//     }
// }
