use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    dir: Direction,
}

fn modulo(divisor: usize, dividend: usize) -> usize {
    // 差为负数则视为逆向读下标
    return (dividend + divisor) % divisor;
}

impl Snake {
    fn new(spawn_index: usize, len: usize) -> Self {
        let mut body = vec![];
        for i in 0..len {
            body.push(SnakeCell(spawn_index - i));
        }
        return Self {
            body,
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
            snake: Snake::new(snake_idx, 3),
        };
    }

    pub fn width(&self) -> usize {
        return self.width;
    }

    pub fn snake_body_idx(&self, idx: usize) -> usize {
        return self.snake.body[idx].0;
    }

    pub fn set_snake_dir(&mut self, dir: Direction) {
        self.snake.dir = dir;
    }

    pub fn snake_len(&self) -> usize {
        return self.snake.body.len();
    }

    // *const 是原始指针
    // 借用规则无用
    pub fn snake_cells(&self) -> *const SnakeCell {
        return self.snake.body.as_ptr();
    }

    // 借用规则阻止给 js 返回引用
    // pub fn snake_cells(&self) -> Vec<SnakeCell> {
    //   return self.snake.body;
    // }

    pub fn step(&mut self) {
        self.snake.body[0] = self.gen_snake_cell();
    }

    pub fn gen_snake_cell(&self) -> SnakeCell {
        let head_idx = self.snake_body_idx(0);
        let row = head_idx / self.width;
        return match self.snake.dir {
            Direction::Right => SnakeCell(row * self.width + modulo(self.width, head_idx + 1)),
            Direction::Left => SnakeCell(row * self.width + modulo(self.width, head_idx - 1)),
            Direction::Up => SnakeCell(modulo(self.size, head_idx - self.width)),
            Direction::Down => SnakeCell(modulo(self.size, head_idx + self.width)),
        };
    }
}
