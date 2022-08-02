//wasm-pack build --target web
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

struct SnakeCell(usize);
//vec
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
pub struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}
impl Snake {
    fn new(spawn_index: usize) -> Snake {
        Snake {
            body: vec![SnakeCell(spawn_index)],
            direction: Direction::Down,
        }
    }
}
#[wasm_bindgen]
pub struct World {
    width: usize,
    snake: Snake,
    size: usize,
}
#[wasm_bindgen]
impl World {
    pub fn new(width: usize, idx: usize) -> World {
        World {
            width,
            size: width * width,
            snake: Snake::new(idx),
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn snake_header(&self) -> usize {
        self.snake.body[0].0
    }
    pub fn update(&mut self) {
        let snake_index = self.snake_header();
        let row = snake_index / self.width;
        let col = snake_index % self.width;

        // 4 5 6 7
        // 4 + ( x + 1) % width

        //取下一列取模
        if self.snake.direction == Direction::Right {
            let row_start = row * self.width;
            let next_col = (snake_index + 1) % self.width;
            self.snake.body[0].0 = row_start + next_col;
        }

        if self.snake.direction == Direction::Left {
            let row_start = row * self.width;
            let next_col = (snake_index - 1) % self.width;
            self.snake.body[0].0 = row_start + next_col;
        }

        //取下一行取模
        if self.snake.direction == Direction::Up {
            let next_row = (row - 1) % self.width;
            self.snake.body[0].0 = next_row * self.width + col;
        }
        if self.snake.direction == Direction::Down {
            let next_row = (row + 1) % self.width;
            self.snake.body[0].0 = next_row * self.width + col;
        }
    }
}
