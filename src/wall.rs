use std::rc::Rc;

use crate::utils::Position;
#[derive(Debug)]
pub struct Wall {
    position: Position,
    width: u16,
    height: u16,
    destroyable: bool,
}
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Direction, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
impl Wall {
    pub fn new(position: Position, size: Position) -> Self {
        Self {
            position,
            width: size.0,
            height: size.1,
            destroyable: false,
        }
    }

    pub fn get_rect(&self) -> Rect {
        Rect {
            x: self.position.0,
            y: self.position.1,
            width: self.width,
            height: self.height,
        }
    }
}
impl Widget for &Wall {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let wall_area = (self).get_rect();
        let block = Block::bordered()
            .border_set(match self.destroyable {
                false => border::ONE_EIGHTH_TALL,
                true => border::LIGHT_DOUBLE_DASHED,
            })
            .gray();
        block.render(wall_area, buf);
    }
}
