use std::collections::VecDeque;

pub type Position = (usize, usize);

#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Bottom,
    Left
}

#[derive(Debug)]
pub struct SnakeGame {
    pub width: usize,
    pub height: usize,
    pub snake: VecDeque<Position>, // head first, tail last
    pub direction:  Direction,
    next_direction: Direction,
    pub food: Position,
    pub finished: bool
}

impl SnakeGame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            snake: [((width - 2).max(0), height / 2)].into_iter().collect(),
            direction: Direction::Left,
            next_direction: Direction::Left,
            food: (2.min(width - 1), height / 2),
            finished: false,
        }
    }

    pub fn change_direction(&mut self, direction:Direction){
        if self.finished {
            return;
        }

        match (&self.direction, direction) {
            (Direction::Up, Direction::Up) |
            (Direction::Up, Direction::Bottom) |
            (Direction::Right, Direction::Right) |
            (Direction::Right, Direction::Left) |
            (Direction::Bottom, Direction::Up) |
            (Direction::Bottom, Direction::Bottom) |
            (Direction::Left, Direction::Right) |
            (Direction::Left, Direction::Left) => {},
            (_, direction) => self.direction = direction,
        }
    }

    pub fn is_valid(&self, (x, y): Position) -> bool {
        x < self.width && y < self.height
    }

    pub fn tick(&mut self) {
        if self.finished && self.snake.len() == 0 {
            return;
        }
        
        let (x, y) = self.snake[0];

        let new_head = match &self.direction {
            Direction::Up => (x, y-1),
            Direction::Right => (x + 1, y),
            Direction::Bottom => (x, y + 1),
            Direction::Left => (x-1, y),
        };

        if !self.is_valid(new_head) || self.snake.contains(&new_head) {
            self.finished = true;
        } else {
            self.snake.pop_back();
            self.snake.push_front(new_head);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SnakeGame;
    #[test]
    fn test() {
        println!("{:?}", SnakeGame::new(10,10));
    }
}