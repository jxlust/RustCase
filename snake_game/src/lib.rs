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
            direction: Direction::Left,
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
        let (row, col) = (snake_index / self.width, snake_index % self.width);
        //左右方向：取下一列(+1/-1)取模
        //上下方向：取下一行(+1/-1)取模
        let (row, col) = match self.snake.direction {
            Direction::Right => (row, (snake_index + 1) % self.width),
            Direction::Left => (row, (snake_index - 1) % self.width),
            Direction::Up => ((row - 1) % self.width, col),
            Direction::Down => ((row + 1) % self.width, col),
        };
        self.snake.body[0].0 = row * self.width + col;
    }
}
