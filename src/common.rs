#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct Position {
    pub(crate) x: u16,
    pub(crate) y: u16,
}

impl Position {
    pub(crate) fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}
