mod snake;
mod common;
mod world;
mod tui;

fn main() -> std::io::Result<()> {
    tui::run()
}
