use crate::bullet::Bullet;
use crate::player::Tank;
use crate::utils::{MovementDirection, Position};
use ratatui::{
    layout::Rect,
    style::Stylize,
    widgets::{Paragraph, Widget},
};
pub struct Enemy {
    pub tank: Tank,
    pub needs_rotation: bool,
}
impl Enemy {
    pub fn new(position: Position) -> Self {
        Self {
            tank: Tank::new(position, 1),
            needs_rotation: false,
        }
    }

    pub fn get_pos(&self) -> (u16, u16) {
        self.tank.position
    }
    pub fn get_rect(&self) -> Rect {
        self.tank.get_rect()
    }
    pub fn get_damage_rect(&self) -> Rect {
        self.tank.get_damage_rect()
    }
    pub fn shoot(&mut self) -> Option<Bullet> {
        if self.tank.health != 0 {
            self.tank.shoot()
        } else {
            None
        }
    }

    pub fn receive_damage(&mut self) {
        if self.tank.health > 0 {
            self.tank.receive_damage(1);
            self.tank.move_forward_steps = 0;
            self.needs_rotation = true;
        }
    }

    pub fn rotate(&mut self) {
        self.tank.rotate();
        self.needs_rotation = false;
    }

    pub fn is_destroyed(&self) -> bool {
        self.tank.health == 0
    }
}
impl Widget for &Enemy {
    fn render(self, _area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
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
