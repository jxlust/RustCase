//wasm-pack build --target web
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/rng.js")]
extern "C" {
    fn rng(n: usize) -> usize;
}
#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(usize);

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Won,
    Lost,
    Played,
}

pub struct Snake {
    //vec
    body: Vec<SnakeCell>,
    direction: Direction,
}
// [10,9,8] 3 length
impl Snake {
    fn new(mut spawn_index: usize, length: usize) -> Snake {
        let mut body = vec![];
        if spawn_index < length {
            spawn_index = length - 1;
        }
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
    next_cell: Option<SnakeCell>,
    food_cell: Option<usize>, //Option handle when the food_cell is None
    status: Option<GameStatus>,
    points: usize,
}
#[wasm_bindgen]
impl World {
    pub fn new(width: usize, idx: usize) -> World {
        let size = width * width;
        let snake = Snake::new(idx, 3);

        World {
            width,
            size,
            food_cell: World::gen_food_cell(size, &snake.body),
            snake,
            next_cell: None,
            status: None,
            points: 0,
        }
    }
    fn gen_food_cell(max: usize, body: &Vec<SnakeCell>) -> Option<usize> {
        let mut food_cell;
        //food may be appear on snakes, then loop fix it
        loop {
            food_cell = rng(max);
            if !body.contains(&SnakeCell(food_cell)) {
                break;
            }
        }
        Some(food_cell)
    }

    pub fn points(&self) -> usize {
        self.points
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn game_status(&self) -> Option<GameStatus> {
        self.status
    }

    pub fn game_status_text(&self) -> String {
        match self.status {
            Some(GameStatus::Won) => String::from("You win!"),
            Some(GameStatus::Lost) => String::from("You Lost!"),
            Some(GameStatus::Played) => String::from("Playing..."),
            None => String::from("Game not start!"),
        }
    }

    pub fn food_cell(&self) -> Option<usize> {
        self.food_cell
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
        // let is_can = match self.snake.direction {
        //     Direction::Right => dir != Direction::Left,
        //     Direction::Left => dir != Direction::Right,
        //     Direction::Up => dir != Direction::Down,
        //     Direction::Down => dir != Direction::Up,
        // };
        // if is_can {
        //     self.snake.direction = dir;
        // }

        //fun 2: next_cell()
        //self.snake.body[1] == next_cell, explain is cant move
        let next_cell = self.gen_next_cell(&dir);
        if self.snake.body[1].0 == next_cell.0 {
            return;
        }
        //Option::Some
        self.next_cell = Some(next_cell);
        self.snake.direction = dir;
    }
    pub fn start_game(&mut self) {
        self.status = Option::Some(GameStatus::Played);
    }

    pub fn step(&mut self) {
        match self.status {
            Some(GameStatus::Played) => {
                let temp = self.snake.body.clone();
                match self.next_cell {
                    Some(cell) => {
                        self.snake.body[0] = cell;
                        self.next_cell = None;
                    }
                    None => {
                        self.snake.body[0] = self.gen_next_cell(&self.snake.direction);
                    }
                }

                let len = self.snake_length();
                for i in 1..len {
                    self.snake.body[i] = SnakeCell(temp[i - 1].0);
                }
                if self.snake.body[1..len].contains(&self.snake.body[0]) {
                    self.status = Some(GameStatus::Lost);
                }

                //todo: eat food
                if Some(self.snake_header()) == self.food_cell {
                    if self.snake_length() < self.size {
                        self.points += 1;
                        //update food cell
                        self.food_cell = World::gen_food_cell(self.size, &self.snake.body);
                    } else {
                        self.food_cell = None; //test
                        self.status = Some(GameStatus::Won);
                        // self.food_cell = self.size + 10;
                    }

                    //length+1,push a cell,cell in snake
                    //eg: 0 < 1 < 2 < 1
                    // x < 0 < 1 < 2 the last is not important
                    //长度+1，push进去的cell理论上在蛇内部的数值就行，只是为了新增一个长度
                    self.snake.body.push(SnakeCell(self.snake.body[1].0));
                }
            }
            //else to do nothing
            _ => {}
        }
    }

    fn gen_next_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_index = self.snake_header();
        let row = snake_index / self.width;
        return match direction {
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
