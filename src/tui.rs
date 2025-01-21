use crate::common::Position;
use crate::snake::Direction;
use crate::world::World;
use crossterm::event::{Event, KeyCode};
use crossterm::{cursor, event, style, terminal, ExecutableCommand};
use std::io::{stdout, Stdout, Write};
use std::time::{Duration, Instant};

pub(crate) fn run() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    stdout
        .execute(terminal::EnterAlternateScreen)
        ?.execute(cursor::Hide)?;

    let (width, height) = terminal::size()?;
    let mut world = World::new(width, height, 3);

    let mut running = true;
    let speed_time_per_action = Duration::from_millis(500); // Default speed is 1 second per action
    let mut last_frame = Instant::now();
    while running {
        while event::poll(Duration::from_millis(10))? {
            if let Event::Key(key_event) = event::read()? {
                let changed_direction = match key_event.code {
                    KeyCode::Char('q') => {
                        running = false;
                        true
                    }, // Exit game
                    KeyCode::Left => world.set_direction(Direction::Left),
                    KeyCode::Right => world.set_direction(Direction::Right),
                    KeyCode::Up => world.set_direction(Direction::Up),
                    KeyCode::Down => world.set_direction(Direction::Down),
                    _ => false,
                };
                
                if changed_direction {
                    update_world(&mut stdout, &mut world)?;
                }
            }
        }

        if last_frame.elapsed() >= speed_time_per_action {
            last_frame = Instant::now();

            update_world(&mut stdout, &mut world)?;
        }
    }

    Ok(())
}

fn update_world(stdout: &mut Stdout, world: &mut World) -> std::io::Result<()> {
    world.make_step();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))?;
    draw_world(stdout, &world)?;

    stdout.flush()
}

fn draw_world(stdout: &mut Stdout, world: &World) -> std::io::Result<()> {
    //print_debug_info(stdout, world)?;
    draw_snake(stdout, world.get_snake_positions())?;
    draw_food(stdout, world.get_food_position())
}

fn draw_snake(stdout: &mut Stdout, positions: Vec<&Position>) -> std::io::Result<()> {
    for position in positions {
        stdout
            .execute(cursor::MoveTo(position.x, position.y))?
            .execute(style::Print("#"))?;
    }

    Ok(())
}

fn draw_food(stdout: &mut Stdout, position: Option<Position>) -> std::io::Result<()> {
    if let Some(position) = position {
        stdout
            .execute(cursor::MoveTo(position.x, position.y))?
            .execute(style::Print("@"))?;
    }
    Ok(())
}

fn print_debug_info(stdout: &mut Stdout, world: &World) -> std::io::Result<()> {
    stdout
        .execute(cursor::MoveTo(0, 0))?
        .execute(style::Print(world.get_debug_info()))?;
    Ok(())
}