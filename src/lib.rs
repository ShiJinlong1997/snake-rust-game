use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[derive(PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    dir: Direction,
}

impl Snake {
    fn new(spawn_index: usize) -> Self {
        return Self {
            body: vec![SnakeCell(spawn_index)],
            dir: Direction::Right,
        };
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> Self {
        return Self {
            width,
            size: width * width,
            snake: Snake::new(snake_idx),
        };
    }

    pub fn width(&self) -> usize {
        return self.width;
    }

    pub fn snake_head_idx(&self) -> usize {
        return self.snake.body[0].0;
    }

    pub fn update(&mut self) {
        let (row, col) = self.index_to_cell(self.snake_head_idx());
        let (row, col) = match self.snake.dir {
            Direction::Right => (row, self.clamp(col + 1) % self.width),
            Direction::Left => (row, self.clamp(col - 1) % self.width),
            Direction::Up => (self.clamp(row - 1) % self.width, col),
            Direction::Down => (self.clamp(row + 1) % self.width, col),
        };

        let next_idx = self.cell_to_index(row, col);
        self.set_snake_head(next_idx);
    }

    fn set_snake_head(&mut self, idx: usize) {
        self.snake.body[0].0 = idx;
    }

    fn clamp(&self, x: usize) -> usize {
        // 避免自减小于 0
        return x + self.width;
    }

    fn index_to_cell(&self, idx: usize) -> (usize, usize) {
        return (idx / self.width, idx % self.width);
    }

    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        return row * self.width + col;
    }
}
