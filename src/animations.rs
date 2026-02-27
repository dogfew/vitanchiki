use ratatui::{
    layout::Rect,
    style::Stylize,
    widgets::{Paragraph, Widget},
};

use crate::{bullet::Bullet, enemy::Enemy, player::Tank, utils::MovementDirection};

pub struct Dummy<'a> {
    paragraph: Paragraph<'a>,
    rect: Rect,
    frames: usize,
}

impl<'a> Dummy<'a> {
    pub fn from_bullet(bullet: &Bullet) -> Self {
        Self {
            paragraph: Paragraph::new("#").red(),
            rect: bullet.get_rect(),
            frames: 10,
        }
    }

    pub fn from_enemy(enemy: &Enemy) -> Self {
        Self {
            paragraph: Paragraph::new(" # \n###\n # ").yellow(),
            rect: enemy.get_rect(),
            frames: 50,
        }
    }

    pub fn from_player(player: &Tank) -> Self {
        Self {
            paragraph: Paragraph::new(" # \n###\n # ").green(),
            rect: player.get_rect(),
            frames: 5000,
        }
    }
    pub fn decrease_frames(&mut self) -> bool {
        if self.frames > 0 {
            self.frames -= 1;
            true
        } else {
            false
        }
    }
}
impl<'a> Widget for &Dummy<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.paragraph.clone().render(self.rect, buf);
    }
}
