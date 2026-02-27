mod animations;
mod bullet;
mod enemy;
mod player;
mod traits;
mod utils;
mod wall;
use bullet::Bullet;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use enemy::Enemy;
use player::Tank;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Clear, Paragraph, Widget, Wrap},
};
use std::{io, time::Duration};
use utils::MovementDirection;
use wall::Wall;

use crate::animations::Dummy;

fn main() -> io::Result<()> {
    loop {
        let mut app = App::default();
        app.walls.push(Wall::new((16, 4), (4, 20)));
        app.walls.push(Wall::new((30, 40), (30, 3)));
        app.walls.push(Wall::new((60, 20), (50, 3)));
        app.walls.push(Wall::new_water((80, 40), (30, 3)));
        app.player.receive_hp(3);
        app.enemies.push(Enemy::new((32, 32)));
        app.enemies.push(Enemy::new((96, 48)));
        app.enemies.push(Enemy::new((70, 15)));
        let result = ratatui::run(|terminal| app.run(terminal));
        if !(app.restart && result.is_ok()) {
            return result;
        }
    }
}

#[derive(Default)]
pub struct App<'a> {
    player: Tank,
    walls: Vec<Wall>,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    dummies: Vec<Dummy<'a>>,
    exit: bool,
    restart: bool,
}
impl<'a> App<'a> {
    // pub fn new() -> Self {
    //     Self {
    //         player: Tank::default(),
    //         bullets: Vec::new(),
    //         enemies: Vec::new(),
    //         walls: Vec::new(),
    //         dummies: Vec::new(),
    //         exit: false,
    //     }
    // }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut i: u32 = 0;

        let size = terminal.size()?;
        let (min_x, min_y) = (2, 2);
        self.player.position.0 = self.player.position.0.max(min_x);
        self.player.position.1 = self.player.position.1.max(min_y);
        let (max_x, max_y) = (size.width, size.height);
        let screen_borders = [
            Rect::new(0, 0, 1, max_y),
            Rect::new(0, 0, max_x, 1),
            Rect::new(0, max_y - 1, max_x, 1),
            Rect::new(max_x - 1, 0, 1, max_y),
        ];
        while !self.exit {
            i = i.wrapping_add(1);
            self.bullets.retain_mut(|bullet| {
                let (x, y) = &mut bullet.position;
                match bullet.direction {
                    MovementDirection::Left => {
                        if *x == 0 {
                            self.dummies.push(Dummy::from_bullet(bullet));
                            return false;
                        } else {
                            *x -= 1;
                        }
                    }
                    MovementDirection::Down => {
                        if *y >= max_y - 1 {
                            self.dummies.push(Dummy::from_bullet(bullet));
                            return false;
                        } else {
                            *y += 1;
                        }
                    }
                    MovementDirection::Up => {
                        if *y == 0 {
                            self.dummies.push(Dummy::from_bullet(bullet));
                            return false;
                        } else {
                            *y -= 1;
                        }
                    }
                    MovementDirection::Right => {
                        if *x >= max_x - 1 {
                            self.dummies.push(Dummy::from_bullet(bullet));
                            return false;
                        } else {
                            *x += 1;
                        }
                    }
                };
                for wall in self.walls.iter().filter(|w| !w.is_water()) {
                    if bullet.get_rect().intersects(wall.get_rect()) {
                        self.dummies.push(Dummy::from_bullet(bullet));
                        return false;
                    }
                }
                for enemy in self.enemies.iter_mut() {
                    if bullet.get_rect().intersects(enemy.get_damage_rect()) {
                        self.dummies.push(Dummy::from_bullet(bullet));
                        enemy.receive_damage();
                        return false;
                    }
                }
                if bullet.get_rect().intersects(self.player.get_damage_rect()) {
                    self.dummies.push(Dummy::from_bullet(bullet));
                    if self.player.health == 1 {
                        self.dummies.push(Dummy::from_player(&self.player));
                    }
                    self.player.receive_damage(1);
                    return false;
                }
                true
            });

            let obstacles: Vec<Rect> = self
                .walls
                .iter()
                .map(|w| w.get_rect())
                .chain(self.enemies.iter().map(|e| e.get_rect()))
                .chain([self.player.get_rect()])
                .chain(screen_borders)
                .collect();
            if self.player.shall_move_forward() {
                self.player.move_forward(&obstacles);
            }
            self.enemies.iter_mut().for_each(|e| {
                if i.is_multiple_of(50) && !e.is_destroyed() {
                    if let Some(bullet) = e.shoot() {
                        self.bullets.push(bullet);
                    }
                } else if i.is_multiple_of(70) {
                    e.tank.rotate();
                    e.tank.move_forward_steps = 5;
                }
                if e.tank.shall_move_forward() {
                    e.tank.move_forward(&obstacles);
                }
            });
            self.enemies.retain(|e| {
                if e.is_destroyed() {
                    self.dummies.push(Dummy::from_enemy(e));
                    false
                } else {
                    true
                }
            });
            self.dummies.retain_mut(|dummy| dummy.decrease_frames());
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(&self.player, frame.area());
        for enemy in &self.enemies {
            frame.render_widget(enemy, frame.area());
        }
        for wall in &self.walls {
            frame.render_widget(wall, frame.area());
        }
        for bullet in &self.bullets {
            frame.render_widget(bullet, frame.area());
        }
        for dummy in self.dummies.iter() {
            frame.render_widget(dummy, frame.area());
        }
        frame.render_widget(self, frame.area());
    }
    // #[disable(warn(clippy::collapsible_if))]
    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                }
                _ => (),
            }
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if self.player.health > 0 && !self.enemies.is_empty() {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => self.exit(),
                KeyCode::Char(' ') | KeyCode::Char('f') => self.bullets.push(self.player.shoot()),
                KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('a') => {
                    self.player.rotate_left()
                }
                KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('d') => {
                    self.player.rotate_right()
                }
                KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => self.player.rotate_up(),
                KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') => {
                    self.player.rotate_down()
                }
                KeyCode::Char('r') => self.player.rotate(),
                KeyCode::Char('R') => self.player.rotate_back(),
                _ => {}
            }
        } else {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => self.exit(),
                KeyCode::Char('r') => self.restart(),
                KeyCode::Char('R') => self.restart(),
                _ => {}
            }
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }
    fn restart(&mut self) {
        self.exit = true;
        self.restart = true;
    }
}

impl<'a> Widget for &App<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" vitanchiki ".bold());
        let left_instructions = Line::from(vec![
            " Left ".into(),
            "<H>".blue().bold(),
            " Down ".into(),
            "<J>".blue().bold(),
            " Up ".into(),
            "<K>".blue().bold(),
            " Right ".into(),
            "<L> ".blue().bold(),
            " Shoot ".into(),
            "<F> ".blue().bold(),
        ]);
        let right_instructions = Line::from(vec![" Exit ".into(), "<Esc/Q> ".blue().bold()]);

        let player_state = Line::from(vec![
            " Health: ".red().bold(),
            (self.player.health).to_string().bold(),
            "  Ammo:  ".green().bold(),
            (self.player.ammo).to_string().bold(),
            " ".into(),
        ]);

        let debug_info = Line::from(vec![
            " Bullets: ".gray(),
            self.bullets.len().to_string().gray(),
            "  Enemies: ".gray(),
            self.enemies.len().to_string().gray(),
            "  Dummies: ".gray(),
            self.dummies.len().to_string().gray(),
            " ".into(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title(player_state)
            .title(debug_info.right_aligned())
            .title_bottom(left_instructions.left_aligned())
            .title_bottom(right_instructions.right_aligned())
            .border_set(border::THICK);
        block.render(area, buf);

        let instructions_left = Line::from(vec![" Restart ".into(), " <R> ".blue().bold()]);
        let instructions_right = Line::from(vec![" Exit ".into(), " <Esc/Q> ".bold().blue()]);
        let block = Block::bordered()
            .title_bottom(instructions_left.left_aligned())
            .title_bottom(instructions_right.right_aligned())
            .border_set(border::PLAIN)
            .style(Style::default().bg(Color::DarkGray))
            .on_black();
        // }
        let area = area.inner(ratatui::layout::Margin {
            horizontal: (25),
            vertical: (25),
        });
        if self.player.health == 0 {
            Clear.render(area, buf);
            Paragraph::new(Line::from("YOU DIED").red().bold())
                .block(block)
                .centered()
                .wrap(Wrap { trim: true })
                .render(area, buf);
        } else if self.enemies.is_empty() {
            Clear.render(area, buf);
            Paragraph::new(Line::from("YOU WON").green().bold())
                .block(block)
                .centered()
                .wrap(Wrap { trim: true })
                .render(area, buf);
        }
    }
}
