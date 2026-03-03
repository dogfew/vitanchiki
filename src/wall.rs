use crate::utils::Position;
use ratatui::{
    layout::Rect,
    style::Stylize,
    symbols::border::ONE_EIGHTH_TALL,
    widgets::{Block, Widget},
};
pub enum WallType {
    Static,
    Breakable(usize),
    Water,
}

pub struct Wall {
    rect: Rect,
    wall_type: WallType,
}
impl Wall {
    pub fn new(position: Position, size: Position) -> Self {
        Self {
            rect: Rect {
                x: position.0,
                y: position.1,
                width: size.0,
                height: size.1,
            },
            wall_type: WallType::Static,
        }
    }

    pub fn new_water(position: Position, size: Position) -> Self {
        Self {
            rect: Rect {
                x: position.0,
                y: position.1,
                width: size.0,
                height: size.1,
            },
            wall_type: WallType::Water,
        }
    }
    pub fn new_destroyable(position: Position, size: Position, health: usize) -> Self {
        Self {
            rect: Rect {
                x: position.0,
                y: position.1,
                width: size.0,
                height: size.1,
            },
            wall_type: WallType::Breakable(health),
        }
    }
    pub fn get_rect(&self) -> Rect {
        self.rect
    }

    pub fn is_water(&self) -> bool {
        matches!(self.wall_type, WallType::Water)
    }
}
impl Widget for &Wall {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let block = match self.wall_type {
            WallType::Static => Block::bordered().border_set(ONE_EIGHTH_TALL).gray(),
            WallType::Water => Block::new().on_blue(),
            WallType::Breakable(_) => Block::bordered().border_set(ONE_EIGHTH_TALL).gray(),
        };
        block.render(self.rect, buf);
    }
}
