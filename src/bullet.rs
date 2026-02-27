use std::alloc::System;

use crate::utils::{MovementDirection, Position};
#[derive(Debug)]
pub struct Bullet {
    pub direction: MovementDirection,
    pub position: Position,
    pub collided: bool,
    pub frames: usize, // animation_frames
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
impl Widget for &Bullet {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let bullet_area = self.get_rect();
        let symbol = {
            if self.collided {
                "#"
            } else {
                match self.direction {
                    MovementDirection::Up | MovementDirection::Down => "|",
                    _ => "—",
                }
            }
        };

        Paragraph::new(symbol).red().render(bullet_area, buf);
    }
}

impl Bullet {
    pub fn get_rect(&self) -> Rect {
        Rect {
            x: self.position.0,
            y: self.position.1,
            width: 1,
            height: 1,
        }
    }
    pub fn is_done(&mut self) -> bool {
        if self.frames == 0 {
            true
        } else {
            self.frames -= 1;
            false
        }
    }
}
