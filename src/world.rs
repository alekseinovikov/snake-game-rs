use crate::common::Position;
use crate::snake::{Direction, Snake};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub(crate) enum WorldState {
    Running,
    GameOver,
}

#[derive(Debug)]
pub(crate) struct World {
    snake: Snake,
    snake_direction: Direction,
    food: Option<Position>,
    width: u16,
    height: u16,
}

impl World {
    pub(crate) fn new(width: u16, height: u16, snake_size: u16) -> Self {
        let x = width / 2;
        let y = height / 2;
        let snake = Snake::new(x, y, snake_size);

        let mut result = Self {
            snake,
            snake_direction: Direction::Right,
            food: None,
            width,
            height,
        };
        result.generate_food_position();
        result
    }

    pub(crate) fn set_direction(&mut self, direction: Direction) -> bool {
        if !self.is_opposite_direction(&direction) {
            self.snake_direction = direction;
            return true;
        }
        
        false
    }

    fn is_opposite_direction(&self, new: &Direction) -> bool {
        matches!(
        (&self.snake_direction, new),
        (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left)
        )
    }

    pub(crate) fn make_step(&mut self) -> WorldState {
        self.snake.move_to(&self.snake_direction);
        if self.snake.intersects(*self.food.as_ref().unwrap()) {
            self.snake.eat();
            self.generate_food_position();
        }

        if self.snake.intersects_itself() {
            WorldState::GameOver
        } else {
            WorldState::Running
        }
    }

    pub(crate) fn get_food_position(&self) -> Option<Position> {
        self.food
    }

    pub(crate) fn get_snake_positions(&self) -> Vec<&Position> {
        self.snake.body.iter().collect()
    }

    pub(crate) fn get_debug_info(&self) -> String {
        format!(
            "Snake positions: {:?},\n\
             Food position: {:?}",
            self.snake.body, self.food
        )
    }

    fn generate_food_position(&mut self) {
        if self.snake.body.len() == (self.width * self.height) as usize {
            self.food = None;
            return;
        }

        loop {
            let x = rand::random::<u16>() % self.width;
            let y = rand::random::<u16>() % self.height;
            let position = Position::new(x, y);
            if !self.snake.intersects(position) {
                self.food = Some(position);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let world = World::new(10, 10, 3);
        assert_eq!(world.width, 10);
        assert_eq!(world.height, 10);
        assert_eq!(world.snake.body.len(), 3);
        assert!(world.food.is_some());
    }

    #[test]
    fn test_generate_food_position() {
        let mut world = World::new(10, 10, 3);
        let snake_head = world.snake.get_head().clone();
        let food = world.food.unwrap();
        assert_ne!(snake_head, food);
    }

    #[test]
    fn test_generate_food_position_when_snake_is_on_the_whole_world() {
        let mut world = World::new(1, 1, 1);
        let snake_head = world.snake.get_head().clone();
        let food = world.food;
        assert_ne!(Some(snake_head), food);
    }

    #[test]
    fn test_generate_food_position_when_snake_is_on_the_whole_world_and_food_is_none() {
        let mut world = World::new(1, 1, 1);
        world.food = None;
        world.generate_food_position();
        assert_eq!(world.food, None);
    }

    #[test]
    fn test_set_direction() {
        let mut world = World::new(10, 10, 3);
        world.set_direction(Direction::Down);
        assert_eq!(world.snake_direction, Direction::Down);
    }

    #[test]
    fn test_make_step() {
        let mut world = World::new(10, 10, 3);
        let snake_head = world.snake.get_head().clone();
        let food = world.food.unwrap();
        world.set_direction(Direction::Down);
        world.make_step();
        assert_ne!(snake_head, world.snake.get_head());
        assert_eq!(world.food, Some(food));
    }

    #[test]
    fn test_make_step_when_snake_eats() {
        let mut world = World::new(10, 10, 3);
        let snake_head = world.snake.get_head().clone();
        let food = world.food.unwrap();
        world.food = Some(Position::new(snake_head.x, snake_head.y + 1));
        world.snake_direction = Direction::Down;
        world.make_step();
        assert_eq!(world.snake.body.len(), 4);
        assert_ne!(snake_head, world.snake.get_head());
        assert_ne!(Some(food), world.food);
    }
}
