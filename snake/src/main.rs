extern crate piston_window;

use std::collections::VecDeque;

use piston_window::*;



#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}


#[derive(Debug, Clone, Copy)]
struct Position {
    x: u32, 
    y: u32,
}

#[derive(Debug)]
struct Snake {
    segments: VecDeque<Position>,
    direction: Direction,
}

const SIZE_BLOCKS: u32 = 20;
const POINTS_PER_BLOCK: u32 = 20;
const SIZE_POINTS: u32 = SIZE_BLOCKS * POINTS_PER_BLOCK;
const BACKGROUND_COLOR : types::Color = [0.0, 0.5, 0.0, 1.0];
const SNAKE_COLOR : types::Color = [1.0, 1.5, 0.0, 1.0];

fn main() {


    let mut window: PistonWindow = WindowSettings::new("snake", [SIZE_POINTS, SIZE_POINTS])
        .exit_on_esc(true)
        .build()
        .unwrap();


    let mut snake = Snake { 
        segments: vec![
            Position {x: 9, y: 9}, 
            Position {x: 9, y: 10}, 
            Position {x: 9, y: 11}, 
        ].into_iter().collect(),
        direction: Direction::Left,
    };

    window.set_ups(2);
    while let Some(e) = window.next() {
        
        if let Some(_) = e.update_args() {
            println!("update");
            // TODO

            // Check if about to collide with wall
            let front = snake.segments.front().unwrap();
            if (snake.direction == Direction::Up && front.y == 0) ||
                (snake.direction == Direction::Down && front.y == SIZE_BLOCKS - 1) ||
                (snake.direction == Direction::Left && front.x == 0) ||
                (snake.direction == Direction::Right && front.x == SIZE_BLOCKS - 1) {
                println!("wall collide");
                continue;
            }

            let mut new_segment = front.clone();
            match snake.direction {
                Direction::Up =>  new_segment.y -= 1,
                Direction::Down =>  new_segment.y += 1,
                Direction::Left =>  new_segment.x -= 1,
                Direction::Right =>  new_segment.x += 1,
            }
            snake.segments.push_front(new_segment);
            snake.segments.pop_back();
        }

        if let Some(_) = e.render_args() {            
            window.draw_2d(&e, |c, g, _d| {
                clear(BACKGROUND_COLOR, g);

                for s in &snake.segments {
                    Rectangle::new(SNAKE_COLOR).draw(
                        [
                            (s.x * POINTS_PER_BLOCK) as f64,
                            (s.y * POINTS_PER_BLOCK) as f64, 
                            POINTS_PER_BLOCK as f64, 
                            POINTS_PER_BLOCK as f64],
                        &c.draw_state,
                        c.transform,
                        g,
                    );    
                }
            });


        }



    }
        
}
