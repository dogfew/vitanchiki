pub type Position = (u16, u16);
#[derive(Debug, Clone, Copy, Default)]
pub enum MovementDirection {
    Left,
    Up,
    Down,
    #[default]
    Right,
}
