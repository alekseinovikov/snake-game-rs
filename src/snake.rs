use std::collections::VecDeque;
use crate::common::Position;

#[derive(Debug)]
pub(crate) struct Snake {
    pub(crate) body: VecDeque<Position>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    pub(crate) fn new(x: u16, y: u16, size: u16) -> Self {
        let mut body = VecDeque::new();
        if size > x {
            body.push_front(Position::new(x, y));
        } else {
            for i in 0..size {
                body.push_front(Position::new(x + i, y));
            }
        }

        Self {
            body
        }
    }

    pub(crate) fn move_to(&mut self, direction: &Direction) {
        let mut new_head = self.body.front().unwrap().clone();
        match direction {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }
        self.body.push_front(new_head);
        self.body.pop_back();
    }

    pub (crate) fn get_head(&self) -> Position {
        self.body.front().unwrap().clone()
    }

    pub (crate) fn eat(&mut self) {
        self.body.push_back(self.body.back().unwrap().clone());
    }

    pub (crate) fn intersects(&self, position: Position) -> bool {
        self.body.iter().any(|&p| p == position)
    }
    
    pub (crate) fn intersects_itself(&self) -> bool {
        let head = self.get_head();
        self.body.iter().skip(1).any(|&p| p == head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let snake = Snake::new(5, 5, 3);
        assert_eq!(snake.body.len(), 3);
    }

    #[test]
    fn test_move_to() {
        let mut snake = Snake::new(5, 5, 3);
        snake.move_to(&Direction::Down);
        assert_eq!(snake.body.len(), 3);
        assert_eq!(snake.get_head().x, 7);
        assert_eq!(snake.get_head().y, 6);
    }

    #[test]
    fn test_eat() {
        let mut snake = Snake::new(5, 5, 3);
        snake.eat();
        snake.move_to(&Direction::Right);
        assert_eq!(snake.body.len(), 4);
        assert_eq!(snake.get_head().x, 8);
        assert_eq!(snake.get_head().y, 5);
    }

    #[test]
    fn test_get_head() {
        let snake = Snake::new(5, 5, 3);
        assert_eq!(snake.get_head().x, 7);
        assert_eq!(snake.get_head().y, 5);
    }
}