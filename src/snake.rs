use crate::common::Position;
use std::collections::VecDeque;

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

        for _ in 0..size {
            body.push_front(Position::new(x, y));
        }

        Self { body }
    }

    pub(crate) fn move_to(&mut self, direction: &Direction, max_x: u16, max_y: u16) {
        let mut new_head = self.body.front().unwrap().clone();
        match direction {
            Direction::Up => {
                if new_head.y == 0 {
                    new_head.y = max_y - 1;
                } else {
                    new_head.y -= 1
                }
            }
            Direction::Down => {
                if new_head.y == max_y - 1 {
                    new_head.y = 0;
                } else {
                    new_head.y += 1
                }
            }
            Direction::Left => {
                if new_head.x == 0 {
                    new_head.x = max_x - 1;
                } else {
                    new_head.x -= 1
                }
            },
            Direction::Right => {
                if new_head.x == max_x - 1 {
                    new_head.x = 0;
                } else {
                    new_head.x += 1
                }
            },
        }

        self.body.push_front(new_head);
        self.body.pop_back();
    }

    pub(crate) fn get_head(&self) -> Position {
        self.body.front().unwrap().clone()
    }

    pub(crate) fn eat(&mut self) {
        self.body.push_back(self.body.back().unwrap().clone());
    }

    pub(crate) fn intersects(&self, position: Position) -> bool {
        self.body.iter().any(|&p| p == position)
    }

    pub(crate) fn intersects_itself(&self) -> bool {
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
        snake.move_to(&Direction::Down, 100, 100);
        assert_eq!(snake.body.len(), 3);
        assert_eq!(snake.get_head().x, 5);
        assert_eq!(snake.get_head().y, 6);
    }

    #[test]
    fn test_eat() {
        let mut snake = Snake::new(5, 5, 3);
        snake.eat();
        snake.move_to(&Direction::Right, 100, 100);
        assert_eq!(snake.body.len(), 4);
        assert_eq!(snake.get_head().x, 6);
        assert_eq!(snake.get_head().y, 5);
    }

    #[test]
    fn test_get_head() {
        let snake = Snake::new(5, 5, 3);
        assert_eq!(snake.get_head().x, 5);
        assert_eq!(snake.get_head().y, 5);
    }

    #[test]
    fn test_intersects() {
        let snake = Snake::new(5, 5, 3);
        assert!(snake.intersects(Position::new(5, 5)));
        assert!(!snake.intersects(Position::new(8, 5)));
    }

    #[test]
    fn test_intersects_itself() {
        let mut snake = Snake::new(5, 5, 5);
        snake.move_to(&Direction::Down, 100, 100);
        snake.move_to(&Direction::Left, 100, 100);
        snake.move_to(&Direction::Up, 100, 100);
        snake.move_to(&Direction::Right, 100, 100);
        assert_eq!(true, snake.intersects_itself());
    }

    #[test]
    fn test_move_to_wrap() {
        let mut snake = Snake::new(5, 5, 3);
        snake.move_to(&Direction::Up, 10, 10);
        assert_eq!(snake.get_head().x, 5);
        assert_eq!(snake.get_head().y, 4);

        snake.move_to(&Direction::Left, 10, 10);
        assert_eq!(snake.get_head().x, 4);
        assert_eq!(snake.get_head().y, 4);

        snake.move_to(&Direction::Down, 10, 10);
        assert_eq!(snake.get_head().x, 4);
        assert_eq!(snake.get_head().y, 5);

        snake.move_to(&Direction::Right, 10, 10);
        assert_eq!(snake.get_head().x, 5);
        assert_eq!(snake.get_head().y, 5);
    }

    #[test]
    fn test_move_to_wrap_negative() {
        let mut snake = Snake::new(0, 0, 3);
        snake.move_to(&Direction::Up, 10, 10);
        assert_eq!(snake.get_head().x, 0);
        assert_eq!(snake.get_head().y, 9);

        snake.move_to(&Direction::Left, 10, 10);
        assert_eq!(snake.get_head().x, 9);
        assert_eq!(snake.get_head().y, 9);

        snake.move_to(&Direction::Down, 10, 10);
        assert_eq!(snake.get_head().x, 9);
        assert_eq!(snake.get_head().y, 0);

        snake.move_to(&Direction::Right, 10, 10);
        assert_eq!(snake.get_head().x, 0);
        assert_eq!(snake.get_head().y, 0);
    }
}
