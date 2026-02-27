pub type Position = (u16, u16);
#[derive(Debug, Clone, Copy, Default)]
pub enum MovementDirection {
    Left,
    Up,
    Down,
    #[default]
    Right,
}

pub fn collides_with(a_pos: Position, b_pos: Position, width: u16, height: u16) -> bool {
    let (x, y) = a_pos;
    let (sx, sy) = b_pos;
    let (ex, ey) = (sx + width, sy + height);
    (x >= sx) && (x <= ex) && (y >= sy) && (y <= ey)
}
