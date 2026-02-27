use crate::bullet::Bullet;
use crate::player::Tank;
use crate::utils::{MovementDirection, Position};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Direction, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
#[derive(Debug)]
pub struct Enemy {
    pub tank: Tank,
    frames: usize,
}

impl Enemy {
    pub fn new(position: Position) -> Self {
        Self {
            tank: Tank::new(position),
            frames: 50,
        }
    }

    pub fn get_rect(&self) -> Rect {
        self.tank.get_rect()
    }
    pub fn get_damage_rect(&self) -> Rect {
        self.tank.get_damage_rect()
    }
    pub fn shoot(&mut self) -> Option<Bullet> {
        if self.tank.health != 0 {
            Some(self.tank.shoot())
        } else {
            None
        }
    }

    pub fn receive_damage(&mut self) {
        if self.tank.health > 0 {
            self.tank.receive_damage();
        }
    }
    pub fn is_destroyed(&self) -> bool {
        self.tank.health == 0
    }
    pub fn is_done(&mut self) -> bool {
        if self.tank.health == 0 {
            if self.frames > 0 {
                self.frames -= 1;
            }
            self.tank.health == 0 && self.frames == 0
        } else {
            false
        }
    }
}
impl Widget for &Enemy {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        if self.tank.health > 0 {
            Paragraph::new(match self.tank.direction {
                MovementDirection::Right => " ▂ \n 𜱴𜱢\n 🮂 ",
                MovementDirection::Up => "   \n▮𜱷▮\n    ",
                MovementDirection::Left => " ▂ \n𜱠𜱶 \n 🮂 ",
                MovementDirection::Down => "   \n▮𜱵▮\n   ",
            })
            .yellow()
            .render(self.tank.get_rect(), buf);
        } else {
            Paragraph::new(match self.tank.direction {
                MovementDirection::Right => " # \n ##\n # ",
                MovementDirection::Up => "   \n###\n    ",
                MovementDirection::Left => " # \n## \n # ",
                MovementDirection::Down => "   \n###\n   ",
            })
            .yellow()
            .render(self.tank.get_rect(), buf);
        }
    }
}
