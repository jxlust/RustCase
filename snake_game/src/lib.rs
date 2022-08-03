//wasm-pack build --target web
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;
#[derive(Clone)]
pub struct SnakeCell(usize);
#[derive(PartialEq)]
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
            direction: Direction::Right,
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

    // pub fn change_ptr(&mut self) {
    //     self.snake.body = vec![SnakeCell(2048)];
    // }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn snake_header(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self, dir: Direction) {
        let is_can = match self.snake.direction {
            Direction::Right => dir != Direction::Left,
            Direction::Left => dir != Direction::Right,
            Direction::Up => dir != Direction::Down,
            Direction::Down => dir != Direction::Up,
        };
        //func2:
        //next_cell()
        //self.snake.body[1] == next_cell, explain is cant move
        if is_can {
            self.snake.direction = dir;
        }
    }

    pub fn step(&mut self) {
        let temp = self.snake.body.clone();
        let next_cell = self.gen_next_cell();
        self.snake.body[0] = next_cell;
        let len = self.snake_length();
        for i in 1..len {
            self.snake.body[i] = SnakeCell(temp[i - 1].0);
        }
    }

    fn gen_next_cell(&self) -> SnakeCell {
        let snake_index = self.snake_header();
        let row = snake_index / self.width;
        return match self.snake.direction {
            Direction::Right => {
                //last_col
                let threshold = (row + 1) * self.width;
                if snake_index + 1 == threshold {
                    SnakeCell(threshold - self.width)
                } else {
                    SnakeCell(snake_index + 1)
                }
            }
            Direction::Left => {
                //first_col
                let threshold = row * self.width;
                if snake_index == threshold {
                    SnakeCell(threshold + self.width - 1)
                } else {
                    SnakeCell(snake_index - 1)
                }
            }
            Direction::Up => {
                //let threshold =  snake_index - row * self.width;
                //first_row
                if row == 0 {
                    SnakeCell(self.size - self.width + snake_index)
                } else {
                    SnakeCell(snake_index - self.width)
                }
            }
            Direction::Down => {
                //last_row
                if row == (self.width - 1) {
                    SnakeCell(self.width - (self.size - snake_index))
                } else {
                    SnakeCell(snake_index + self.width)
                }
            }
        };
    }

    // fn set_snake_header(&mut self, idx: usize) {
    //     self.snake.body[0].0 = idx;
    // }

    // fn index_to_cell(&self, idx: usize) -> (usize, usize) {
    //     (idx / self.width, idx % self.width)
    // }

    // //x,y (col,row) => index
    // fn cell_to_index(&self, row: usize, col: usize) -> usize {
    //     row * self.width + col
    // }
}
