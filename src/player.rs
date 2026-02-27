use std::ptr::with_exposed_provenance_mut;

use crate::bullet::Bullet;
// use crate::traits::Tank;
use crate::utils::{MovementDirection, Position};
use ratatui::symbols::border::HEAVY_DOUBLE_DASHED;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Direction, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
#[derive(Debug, Default)]
pub struct Tank {
    pub position: Position,
    pub direction: MovementDirection,
    pub health: usize,
    pub ammo: usize,
    pub move_forward_steps: usize,
}

impl Tank {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            direction: MovementDirection::default(),
            health: 3,
            ammo: 1000,
            move_forward_steps: 0,
        }
    }
}

impl Widget for &Tank {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new(match self.direction {
            MovementDirection::Right => " ▂ \n 𜱴𜱢\n 🮂 ",
            MovementDirection::Up => "   \n▮𜱷▮\n    ",
            MovementDirection::Left => " ▂ \n𜱠𜱶 \n 🮂 ",
            MovementDirection::Down => "   \n▮𜱵▮\n   ",
        })
        .green()
        .render(self.get_rect(), buf);
    }
}

impl Tank {
    pub fn shall_move_forward(&mut self) -> bool {
        let res = self.move_forward_steps > 0;
        if res {
            self.move_forward_steps -= 1;
        }
        res
    }

    pub fn move_forward(&mut self, obstacles: &Vec<Rect>) {
        let obstacles = obstacles.clone();
        match self.direction {
            MovementDirection::Left => self.move_left(obstacles),
            MovementDirection::Up => self.move_up(obstacles),
            MovementDirection::Right => self.move_right(obstacles),
            MovementDirection::Down => self.move_down(obstacles),
        }
    }
    pub fn move_up(&mut self, obstacles: Vec<Rect>) {
        match self.direction {
            MovementDirection::Up => {
                self.position.1 = self.position.1.saturating_sub(1);
                for obstacle in obstacles {
                    if self.get_rect().intersects(obstacle) {
                        self.position.1 += 1;
                    }
                }
            }
            _ => {
                self.direction = MovementDirection::Up;
            }
        }
    }
    pub fn move_down(&mut self, obstacles: Vec<Rect>) {
        match self.direction {
            MovementDirection::Down => {
                self.position.1 = self.position.1.saturating_add(1);
                for obstacle in obstacles {
                    if self.get_rect().intersects(obstacle) {
                        self.position.1 -= 1;
                        return;
                    }
                }
            }
            _ => {
                self.direction = MovementDirection::Down;
            }
        }
    }

    pub fn move_right(&mut self, obstacles: Vec<Rect>) {
        match self.direction {
            MovementDirection::Right => {
                self.position.0 = self.position.0.saturating_add(1);
                for obstacle in obstacles {
                    if self.get_rect().intersects(obstacle) {
                        self.position.0 -= 1;
                        return;
                    }
                }
            }
            _ => {
                self.direction = MovementDirection::Right;
            }
        }
    }
    pub fn move_left(&mut self, obstacles: Vec<Rect>) {
        match self.direction {
            MovementDirection::Left => {
                self.position.0 = self.position.0.saturating_sub(1);
                for obstacle in obstacles {
                    if self.get_rect().intersects(obstacle) {
                        self.position.0 += 1;
                        return;
                    }
                }
            }
            _ => {
                self.direction = MovementDirection::Left;
            }
        }
    }

    pub fn rotate_right(&mut self) {
        if let MovementDirection::Right = self.direction {
            self.move_forward_steps += 1
        } else {
            self.move_forward_steps = 0;
            self.direction = MovementDirection::Right;
        }
    }
    pub fn rotate_left(&mut self) {
        if let MovementDirection::Left = self.direction {
            self.move_forward_steps += 1
        } else {
            self.move_forward_steps = 0;
            self.direction = MovementDirection::Left;
        }
    }
    pub fn rotate_up(&mut self) {
        if let MovementDirection::Up = self.direction {
            self.move_forward_steps += 1;
        } else {
            self.direction = MovementDirection::Up;
            self.move_forward_steps = 0;
        }
    }
    pub fn rotate_down(&mut self) {
        if let MovementDirection::Down = self.direction {
            self.move_forward_steps += 1
        } else {
            self.direction = MovementDirection::Down;
            self.move_forward_steps = 0
        }
    }
    pub fn rotate(&mut self) {
        self.direction = {
            match self.direction {
                MovementDirection::Left => MovementDirection::Down,
                MovementDirection::Up => MovementDirection::Left,
                MovementDirection::Down => MovementDirection::Right,
                MovementDirection::Right => MovementDirection::Up,
            }
        }
    }

    pub fn rotate_back(&mut self) {
        self.direction = {
            match self.direction {
                MovementDirection::Down => MovementDirection::Left,
                MovementDirection::Left => MovementDirection::Up,
                MovementDirection::Right => MovementDirection::Down,
                MovementDirection::Up => MovementDirection::Right,
            }
        }
    }
    pub fn shoot(&mut self) -> Bullet {
        let direction = self.direction;
        let (x, y) = self.position;
        let position: Position = match direction {
            MovementDirection::Left => (x, y + 1),
            MovementDirection::Right => (x + 2, y + 1),
            MovementDirection::Up => (x + 1, y),
            MovementDirection::Down => (x + 1, y + 1),
        };
        self.move_forward_steps = 0;
        Bullet {
            direction,
            position,
            collided: false,
            frames: 5,
        }
    }
    pub fn get_rect(&self) -> Rect {
        let (x, y) = self.position;
        Rect {
            x,
            y,
            width: 3,
            height: 3,
        }
    }
    pub fn get_damage_rect(&self) -> Rect {
        let (x, y) = self.position;

        match self.direction {
            MovementDirection::Down | MovementDirection::Up => Rect {
                x,
                y: y + 1,
                width: 3,
                height: 1,
            },
            MovementDirection::Left | MovementDirection::Right => Rect {
                x,
                y,
                width: 3,
                height: 3,
            },
        }
    }
}

impl Tank {
    pub fn receive_hp(&mut self) {
        self.health = self.health.saturating_add(1);
    }
    pub fn receive_damage(&mut self) {
        self.health = self.health.saturating_sub(1);
    }
}
const PLAYER_LEFT: &str = "  ▂  \n  𜱴𜱢\n  🮂";
const PLAYER_UP: &str = "   \n ▮𜱷▮ \n   ";
const PLAYER_RIGHT: &str = "  ▂  \n 𜱠𜱶 \n  🮂";
const PLAYER_DOWN: &str = "   \n ▮𜱵▮ \n   ";
