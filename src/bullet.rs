use crate::utils::{MovementDirection, Position};
#[derive(Copy, Clone)]
pub struct Bullet {
    pub position: Position,
    pub direction: MovementDirection,
}
use ratatui::{
    layout::Rect,
    style::Stylize,
    widgets::{Paragraph, Widget},
};
impl Widget for &Bullet {
    fn render(self, _area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let symbol = {
            match self.direction {
                MovementDirection::Up | MovementDirection::Down => "|",
                MovementDirection::Left | MovementDirection::Right => "—",
            }
        };
        Paragraph::new(symbol).red().render(self.get_rect(), buf);
    }
}

impl Bullet {
    pub fn new(position: Position, direction: MovementDirection) -> Self {
        Self {
            position,
            direction,
        }
    }

    pub fn get_rect(&self) -> Rect {
        let (x, y) = self.position;
        Rect {
            x,
            y,
            width: 1,
            height: 1,
        }
    }
}
