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
        let snake_idx = self.snake_head_idx();
        // 0 <= 下标范围 < 64
        // self.snake.body[0].0 = (snake_idx - 1) % self.size;

        if (Direction::Right == self.snake.dir) {
            self.snake.body[0].0 = (snake_idx + 1) % self.size;
        } else if (Direction::Left == self.snake.dir) {
            self.snake.body[0].0 = (snake_idx - 1) % self.size;
        } else if (Direction::Up == self.snake.dir) {
            self.snake.body[0].0 = (snake_idx - self.width) % self.size;
        } else if (Direction::Down == self.snake.dir) {
            self.snake.body[0].0 = (snake_idx + self.width) % self.size;
        }
    }
}
