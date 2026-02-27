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
    layout::{Direction, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::{io, time::Duration};
use utils::{MovementDirection, Position};
use wall::Wall;

fn main() -> io::Result<()> {
    let mut app = App::new();
    app.walls.push(Wall::new((16, 4), (4, 20)));
    app.walls.push(Wall::new((30, 40), (30, 3)));
    app.player.receive_hp();
    app.enemies.push(Enemy::new((32, 32)));
    ratatui::run(|terminal| app.run(terminal))
}

#[derive(Debug, Default)]
pub struct App {
    player: Tank,
    walls: Vec<Wall>,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    exit: bool,
}
impl App {
    pub fn new() -> Self {
        Self {
            player: Tank::default(),
            bullets: Vec::new(),
            enemies: Vec::new(),
            walls: Vec::new(),
            exit: false,
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut i: u32 = 0;

        let size = terminal.size()?;
        let (min_x, min_y) = (2, 2);
        self.player.position.0 = self.player.position.0.max(min_x);
        self.player.position.1 = self.player.position.1.max(min_y);
        let (max_x, max_y) = (size.width, size.height);
        while !self.exit {
            // Process bullet movement
            self.bullets.retain_mut(|bullet| {
                let (x, y) = &mut bullet.position;
                if bullet.collided {
                    return !bullet.is_done();
                }
                match bullet.direction {
                    MovementDirection::Left => {
                        if *x == 0 {
                            bullet.collided = true;
                        } else {
                            *x -= 1;
                        }
                    }
                    MovementDirection::Down => {
                        if *y >= max_y {
                            bullet.collided = true;
                        } else {
                            *y += 1;
                        }
                    }
                    MovementDirection::Up => {
                        if *y <= min_y {
                            bullet.collided = true;
                        } else {
                            *y -= 1;
                        }
                    }
                    MovementDirection::Right => {
                        if *x >= max_x {
                            bullet.collided = true;
                        } else {
                            *x += 1;
                        }
                    }
                };
                for wall in &self.walls {
                    if bullet.get_rect().intersects(wall.get_rect()) {
                        bullet.collided = true;
                    }
                }
                for enemy in self.enemies.iter_mut() {
                    if bullet.get_rect().intersects(enemy.get_damage_rect()) {
                        bullet.collided = true;
                        enemy.receive_damage();
                    }
                }
                if bullet.get_rect().intersects(self.player.get_damage_rect()) {
                    bullet.collided = true;
                    self.player.receive_damage();
                }
                true
            });

            let obstacles: Vec<Rect> = self
                .walls
                .iter()
                .map(|w| w.get_rect())
                // .chain(self.enemies.iter().map(|e| e.get_rect()))
                .collect();

            let cloned_obstacles = obstacles.clone();
            if self.player.shall_move_forward() {
                self.player.move_forward(&obstacles);
            }
            i = i.wrapping_add(1);
            self.enemies.retain_mut(|e| {
                if i.is_multiple_of(50) && !e.is_destroyed() {
                    if let Some(bullet) = e.shoot() {
                        self.bullets.push(bullet);
                    }
                } else if i.is_multiple_of(70) {
                    e.tank.rotate();
                    e.tank.move_forward_steps = 10;
                }
                if e.tank.shall_move_forward() {
                    e.tank.move_forward(&obstacles);
                }

                !e.is_done()
            });
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
    }
    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key_event) = event::read().unwrap()
                && key_event.kind == KeyEventKind::Press
            {
                self.handle_key_event(key_event);
            } else {
                println!("Hello!!!");
            }
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(' ') | KeyCode::Char('f') => self.bullets.push(self.player.shoot()),
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('a') => self.player.rotate_left(),
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('d') => self.player.rotate_right(),
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => self.player.rotate_up(),
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') => self.player.rotate_down(),
            KeyCode::Char('r') => self.player.rotate(),
            KeyCode::Char('R') => self.player.rotate_back(),
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
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
            " Ammo:  ".green().bold(),
            (self.player.ammo).to_string().bold(),
            " ".into(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title(player_state)
            .title_bottom(left_instructions.left_aligned())
            .title_bottom(right_instructions.right_aligned())
            .border_set(border::THICK);
        block.render(area, buf);
    }
}
