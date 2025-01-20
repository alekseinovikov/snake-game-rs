use crate::common::Position;
use crate::snake::Snake;

#[derive(Debug)]
pub(crate) struct World {
    snake: Snake,
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
            food: None,
            width,
            height,
        };
        result.generate_food_position();
        result
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
}