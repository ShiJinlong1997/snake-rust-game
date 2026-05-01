use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/util/rnd.js")]
extern "C" {
    fn rnd(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy)]
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
    Playing,
}

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy)]
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
    fn new(head_idx: usize, len: usize) -> Self {
        let mut body = vec![];
        for i in 0..len {
            body.push(SnakeCell(head_idx - i));
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
    moved_head: Option<SnakeCell>,
    reward_idx: Option<usize>,
    status: Option<GameStatus>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_head_idx: usize) -> Self {
        let snake = Snake::new(snake_head_idx, 3);
        let size = width * width;

        return Self {
            width,
            size,
            reward_idx: World::gen_reward_idx(size, &snake.body),
            snake,
            moved_head: Option::None,
            status: Option::None,
        };
    }

    fn gen_reward_idx(max: usize, snake_body: &Vec<SnakeCell>) -> Option<usize> {
        let mut idx: usize;
        loop {
            idx = rnd(max);
            if !snake_body.contains(&SnakeCell(idx)) {
                break;
            }
        }
        return Option::Some(idx);
    }

    pub fn width(&self) -> usize {
        return self.width;
    }

    pub fn reward_idx(&self) -> Option<usize> {
        return self.reward_idx;
    }

    pub fn snake_body_idx(&self, idx: usize) -> usize {
        return self.snake.body[idx].0;
    }

    pub fn snake_len(&self) -> usize {
        return self.snake.body.len();
    }

    // *const 是原始指针
    // 借用规则无用
    pub fn snake_cells(&self) -> *const SnakeCell {
        return self.snake.body.as_ptr();
    }

    pub fn status(&self) -> Option<GameStatus> {
        return self.status;
    }

    pub fn status_txt(&self) -> String {
        return match self.status {
            Option::Some(GameStatus::Won) => String::from("赢了"),
            Option::Some(GameStatus::Lost) => String::from("废了"),
            Option::Some(GameStatus::Playing) => String::from("在玩"),
            Option::None => String::from("未知"),
        };
    }

    // 借用规则阻止给 js 返回引用
    // pub fn snake_cells(&self) -> Vec<SnakeCell> {
    //   return self.snake.body;
    // }

    pub fn start(&mut self) {
        self.status = Option::Some(GameStatus::Playing);
    }

    pub fn set_snake_dir(&mut self, dir: Direction) {
        // 防止蛇回头
        let moved_head = self.gen_moved_head(dir);
        if moved_head.0 == self.snake.body[1].0 {
            return;
        }

        self.moved_head = Option::Some(moved_head);
        self.snake.dir = dir;
    }

    pub fn step(&mut self) {
        match self.status {
            Option::Some(GameStatus::Playing) => {
                // 蛇头移动前创建出蛇的副本
                let old_body = self.snake.body.clone();

                match self.moved_head {
                    Option::Some(cell) => {
                        self.snake.body[0] = cell;
                        self.moved_head = Option::None;
                    }
                    Option::None => {
                        self.snake.body[0] = self.gen_moved_head(self.snake.dir);
                    }
                }

                // 从下标 1 开始，所以身体部分，移到之前前一个身体部分的位置
                for i in 1..self.snake_len() {
                    self.snake.body[i] = SnakeCell(old_body[i - 1].0);
                }

                if self.snake.body[1..self.snake_len()].contains(&self.snake.body[0]) {
                    self.status = Option::Some(GameStatus::Lost);
                }

                // 吃到奖品
                if self.reward_idx == Option::Some(self.snake_body_idx(0)) {
                    self.snake.body.push(SnakeCell(self.snake.body[1].0));
                    if self.snake_len() < self.size {
                        self.reward_idx = World::gen_reward_idx(self.size, &self.snake.body);
                    } else {
                        self.reward_idx = Option::None;
                        self.status = Option::Some(GameStatus::Won);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn gen_moved_head(&self, dir: Direction) -> SnakeCell {
        let head_idx = self.snake_body_idx(0);
        let row = head_idx / self.width;
        return match dir {
            Direction::Right => SnakeCell(row * self.width + modulo(self.width, head_idx + 1)),
            Direction::Left => SnakeCell(row * self.width + modulo(self.width, head_idx - 1)),
            Direction::Up => SnakeCell(modulo(self.size, head_idx - self.width)),
            Direction::Down => SnakeCell(modulo(self.size, head_idx + self.width)),
        };
    }
}
