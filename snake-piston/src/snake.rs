use std::collections::LinkedList;
use piston::input::keyboard::Key;
use rand;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN
}


#[derive(Copy, Clone, PartialEq)]
pub struct Block {
    pub x: u32,
    pub y: u32
}

pub struct Snake {
    pub alive: bool,
    dir: Direction,
    dir_lock: bool,
    pub snake: LinkedList<Block>,
    pub candy: Block,
    growth: u32,
    grid_width: u32,
    grid_height: u32,
    pub current_score: u32,
    pub high_score: u32,
    pub paused: bool
}

impl Snake {
    pub fn new(grid_width: u32, grid_height: u32) -> Snake{
        let mut s = Snake {
            alive: true,
            dir: Direction::RIGHT,
            candy: Block {x: grid_width / 2 , y: (grid_height / 2) + 5},
            snake: {
                let mut new_snake: LinkedList<Block> = LinkedList::new();
                new_snake.push_front(Block {x: grid_width / 2, y: grid_height / 2});
                new_snake
            },
            growth: 0,
            grid_width: grid_width,
            grid_height: grid_height,
            current_score: 0,
            high_score: 0,
            paused: true,
            dir_lock: false,
        };
        s.get_candy();
        s
    }

    pub fn reset(&mut self) {
        self.alive = true;
        self.dir = Direction::RIGHT;
        self.snake =  {
            let mut new_snake: LinkedList<Block> = LinkedList::new();
            new_snake.push_front(Block {x: self.grid_width / 2, y: self.grid_height / 2});
            new_snake };
        self.growth = 0;
        self.current_score =0;
        self.paused = true;
    }

    fn reverse(d: Direction ) -> Direction {
        match d {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::RIGHT => Direction::LEFT,
            _ => Direction::RIGHT
        }
    }

    pub fn get_candy(&mut self) {
        use rand::Rng;
        self.candy = Block {
            x: rand::thread_rng().gen_range(0, self.grid_width),
            y: rand::thread_rng().gen_range(0, self.grid_height)
        };
        let mut failed = false;
        for b in self.snake.iter_mut() {
            if b.clone() == self.candy.clone() {
                failed = true;
            }
        }

        if failed  {
             self.get_candy();
         }
    }
    pub fn pause(&mut self) {
        self.paused = !self.paused;
    }
    pub fn change_direction(&mut self, k: Key) {
        use piston::input::keyboard::Key::*;
        if self.dir_lock { return }
        let dir = self.dir;
        let new_dir  = match k {
            Up => Direction::UP,
            Down => Direction::DOWN,
            Left => Direction::LEFT,
            Right => Direction::RIGHT,
            _ => dir
        };
        if new_dir == Snake::reverse(dir) {
            return
        }
        self.dir = new_dir;
        self.dir_lock = true;
    }

    pub fn tick (&mut self) -> bool {
        let front: Block = self.snake.front().unwrap().clone();
        self.dir_lock = false;
        let dir = self.dir;
        let candy = self.candy;
        //bounds checking
        // This code didn't work so I had to hard code it.
        //let limit = self.grid_width - 1;

        self.alive = match (dir, front.x) {
            (Direction::LEFT, 0) => false,
            (Direction::RIGHT,  59 /* TODO make dynamic */) => false,
            _ => true,
        };
        if !self.alive { return false }
        //let limit = self.grid_height - 1;
        self.alive = match (dir, front.y) {
            (Direction::UP, 0) => false,
            (Direction::DOWN, 59 /* TODO make dynamics */) => false,
            _ => true
        };
        if !self.alive { return false }
        //move foward
        let next_move = Block {
                x: match dir {
                    Direction::LEFT => (front.x - 1),
                    Direction::RIGHT => (front.x + 1),
                    _ => front.x
                },
                y: match dir {
                    Direction::UP => (front.y - 1),
                    Direction::DOWN => (front.y + 1),
                    _ => front.y
                }
            };
        // does the snake run into itself?
        for b in self.snake.iter_mut() {
            if b.clone() == next_move {
                self.alive = false;
                return false;
            }
        }

        self.snake.push_front(next_move);

        if self.growth == 0 {
            self.snake.pop_back();
        } else {
            self.growth-=1;
        }

        let front: Block = self.snake.front().unwrap().clone();
        if front == candy {
            self.growth +=3;
            self.current_score += 1;
            self.get_candy();
        }
        if self.current_score > self.high_score {
            self.high_score = self.current_score;
        }
        true
    }
}
