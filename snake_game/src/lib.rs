//wasm-pack build --target web
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

pub struct SnakeCell(usize);
// #[derive(PartialEq)]
#[wasm_bindgen]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
pub struct Snake {
    //vec
    body: Vec<SnakeCell>,
    direction: Direction,
}
// [10,9,8] 3 length
impl Snake {
    fn new(spawn_index: usize, length: usize) -> Snake {
        let mut body = vec![];
        for i in 0..length {
            body.push(SnakeCell(spawn_index - i))
        }
        Snake {
            body,
            direction: Direction::Up,
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
            snake: Snake::new(idx, 3),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    //cannot return a reference to JS because of borring rules
    //cannot return a borrowed ref with #[wasm_bindgen]rustc
    // pub fn snake_cells(&self) -> &Vec<SnakeCell>{
    //     &self.snake.body
    // }

    //*const is raw pointer
    //borrowing rules doesn't apply to it
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn change_ptr(&mut self) {
        self.snake.body = vec![SnakeCell(2048)];
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn snake_header(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self, dir: Direction) {
        self.snake.direction = dir;
    }

    pub fn update(&mut self) {
        let snake_index = self.snake_header();
        let (row, col) = self.index_to_cell(snake_index);
        //左右方向：取下一列(+1/-1)取模
        //上下方向：取下一行(+1/-1)取模
        let (row, col) = match self.snake.direction {
            Direction::Right => (row, (snake_index + 1) % self.width),
            Direction::Left => (row, (snake_index - 1) % self.width),
            Direction::Up => ((row - 1) % self.width, col),
            Direction::Down => ((row + 1) % self.width, col),
        };
        let idx = self.cell_to_index(row, col);
        self.set_snake_header(idx);
    }

    fn set_snake_header(&mut self, idx: usize) {
        self.snake.body[0].0 = idx;
    }

    fn index_to_cell(&self, idx: usize) -> (usize, usize) {
        (idx / self.width, idx % self.width)
    }

    //x,y (col,row) => index
    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
}
