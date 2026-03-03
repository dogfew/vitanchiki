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
use std::{
    io,
    thread::sleep,
    time::{Duration, Instant},
};
use utils::MovementDirection;
use wall::Wall;

use crate::{animations::Dummy, bullet, enemy, player, utils, wall};
pub struct App<'a> {
    pub player: Tank,
    pub walls: Vec<Wall>,
    pub enemies: Vec<Enemy>,
    pub bullets: Vec<Bullet>,
    pub dummies: Vec<Dummy<'a>>,
    difficulty: Difficulty,
    state: AppState,
    lvl: u16,
}
pub enum AppState {
    Playing,
    Restart,
    Exit,
}
enum Difficulty {
    Easy,
    Hard,
}

const FRAME_DURATION: Duration = Duration::from_nanos(16_666_666);
const ENEMY_SHOOT_INTERVAL: u32 = 55;
const AMMO_INTERVAL: u32 = 100;
const ENEMY_ROTATION_INTERVAL: u32 = 90;
impl<'a> App<'a> {
    pub fn new_random(
        terminal: &DefaultTerminal,
        n_walls: u16,
        n_enemies: u16,
        factor: u16,
        div_factor: u16,
        lvl: u16,
        is_hard: bool,
    ) -> io::Result<Self> {
        let size = terminal.size()?;
        let max_ammo = if is_hard { 3 } else { 5 };
        let mut app = App {
            player: Tank::new((2, 2), max_ammo),
            walls: Vec::new(),
            enemies: Vec::new(),
            bullets: Vec::new(),
            dummies: Vec::new(),
            difficulty: if is_hard {
                Difficulty::Hard
            } else {
                Difficulty::Easy
            },
            state: AppState::Playing,
            // exit: false,
            // restart: false,
            lvl,
        };
        let (width, height) = (size.width, size.height);
        let n = (n_walls.max(n_enemies) as f32).sqrt().ceil() as u16;
        let (block_width, block_height) = (width / n, height / n);
        let (mut x, mut y) = (0, block_height / 2);
        for i in 0..n_walls {
            let (position, size) = if i.is_multiple_of(div_factor) {
                ((x, y), (block_width * factor / (factor + 1), 2))
            } else {
                (
                    (x, y - (block_height / 2)),
                    (6, block_height * factor / (factor + 1)),
                )
            };
            if (i + 1).is_multiple_of(4) {
                app.walls.push(Wall::new_water(position, size));
            } else {
                app.walls.push(Wall::new(position, size));
            }
            if (i + 1).is_multiple_of(n) {
                x = 0;
                y += block_height;
            } else {
                x += block_width
            }
        }
        (x, y) = (width, height);
        for i in 0..n_enemies {
            app.enemies
                .push(Enemy::new((x - (block_width / 3), y - (block_height / 3))));
            if (i + 1).is_multiple_of(n) {
                x = width;
                y -= block_height;
            } else {
                x -= block_width
            }
        }
        Ok(app)
    }
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
        let is_hard = self.is_hard();
        while matches!(self.state, AppState::Playing) {
            let timer = Instant::now();
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
                if !e.is_destroyed() {
                    let looks_vertical = e.tank.position.0.abs_diff(self.player.position.0) <= 1;
                    let looks_horizontal = e.tank.position.1.abs_diff(self.player.position.1) <= 1;
                    if i.is_multiple_of(ENEMY_SHOOT_INTERVAL) {
                        e.tank.ammo = 1;
                    };
                    if looks_vertical || looks_horizontal {
                        // Rotate tank
                        if looks_vertical {
                            if self.player.position.1 > e.tank.position.1 {
                                e.tank.rotate_down();
                            } else {
                                e.tank.rotate_up();
                            }
                        }
                        if looks_horizontal {
                            if self.player.position.0 > e.tank.position.0 {
                                e.tank.rotate_right();
                            } else {
                                e.tank.rotate_left();
                            }
                        }
                        // Perform actions
                        if (is_hard || i.is_multiple_of(ENEMY_SHOOT_INTERVAL))
                            && let Some(bullet) = e.shoot()
                        {
                            self.bullets.push(bullet);
                        } else if i.is_multiple_of(ENEMY_ROTATION_INTERVAL) {
                            e.tank.move_forward_steps += 1;
                            e.tank.move_forward(&obstacles);
                        }
                    } else if e.tank.shall_move_forward() {
                        e.tank.move_forward(&obstacles);
                    } else if i.is_multiple_of(ENEMY_ROTATION_INTERVAL) || e.needs_rotation {
                        e.rotate();
                        e.tank.move_forward_steps = 5
                    }
                }
            });

            if i.is_multiple_of(AMMO_INTERVAL) {
                self.player.add_ammo();
            }
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
            self.handle_events(timer.elapsed())?;
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
        for dummy in &self.dummies {
            frame.render_widget(dummy, frame.area());
        }
        frame.render_widget(self, frame.area());
    }
    // #[disable(warn(clippy::collapsible_if))]
    fn handle_events(&mut self, passed: Duration) -> io::Result<()> {
        let now = Instant::now();
        if event::poll(FRAME_DURATION.saturating_sub(passed))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                }
                _ => (),
            }
        };
        sleep(
            FRAME_DURATION
                .saturating_sub(now.elapsed())
                .saturating_sub(passed),
        );
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if !self.is_game_end() {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => self.exit(),
                KeyCode::Char(' ') | KeyCode::Char('f') => {
                    if let Some(bullet) = self.player.shoot() {
                        self.bullets.push(bullet);
                    }
                }
                // KeyCode::Char('g') => {
                //     if let Some(bullet) = self.player.shoot() {
                //         self.bullets.push(bullet);
                //     }
                // }
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
                KeyCode::Enter
                | KeyCode::Char('r')
                | KeyCode::Char('R')
                | KeyCode::Char('N')
                | KeyCode::Char('n') => self.restart(),
                KeyCode::Char('m') | KeyCode::Char('M') => match self.difficulty {
                    Difficulty::Easy => self.difficulty = Difficulty::Hard,
                    Difficulty::Hard => self.difficulty = Difficulty::Easy,
                },
                _ => {}
            }
        }
    }

    pub fn is_lost(&self) -> bool {
        self.player.health == 0
    }
    fn is_game_end(&self) -> bool {
        self.player.health == 0 || self.enemies.is_empty()
    }

    fn exit(&mut self) {
        self.state = AppState::Exit
    }
    fn restart(&mut self) {
        self.state = AppState::Restart
    }
    pub fn is_restarting(&self) -> bool {
        matches!(self.state, AppState::Restart)
    }
    pub fn is_hard(&self) -> bool {
        matches!(self.difficulty, Difficulty::Hard)
    }
}

impl<'a> Widget for &App<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title_vec = vec![
            " vitanchiki ".bold(),
            " level ".into(),
            self.lvl.to_string().into(),
            " ".into(),
        ];
        let title = Line::from(if matches!(self.difficulty, Difficulty::Hard) {
            title_vec.iter().map(|x| x.clone().bold().red()).collect()
        } else {
            title_vec
        });
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
        Block::bordered()
            .title(title.centered())
            .title(player_state)
            .title(debug_info.right_aligned())
            .title_bottom(left_instructions.left_aligned())
            .title_bottom(right_instructions.right_aligned())
            .border_set(border::THICK)
            .render(area, buf);
        if self.is_game_end() {
            let instructions_left = Line::from(vec![
                (if self.enemies.is_empty() {
                    " Next "
                } else {
                    " Retry "
                })
                .into(),
                " <R/N> ".blue().bold(),
            ]);
            let instructions_right = Line::from(vec![" Exit ".into(), " <Esc/Q> ".bold().blue()]);
            let title = Line::from(vec![
                (if self.is_hard() {
                    " Hard ".red()
                } else {
                    " Easy ".green()
                }),
                " <M> ".bold().blue(),
            ]);
            let block = Block::bordered()
                .title_bottom(title.centered())
                .title_bottom(instructions_left.left_aligned())
                .title_bottom(instructions_right.right_aligned())
                .border_set(border::PLAIN)
                .style(Style::default().bg(Color::DarkGray))
                .on_black();
            // }
            // let area = area.inner(ratatui::layout::Margin {
            //     horizontal: (6),
            //     vertical: (6),
            // });
            let area = ratatui::layout::Rect::centered_vertically(
                area,
                ratatui::layout::Constraint::Ratio(1, 4),
            );
            let area = ratatui::layout::Rect::centered_horizontally(
                area,
                ratatui::layout::Constraint::Ratio(1, 2),
            );
            let text = if self.is_lost() {
                Line::from("\nYOU DIED").red().bold()
            } else {
                Line::from("\nYOU WON").green().bold()
            };
            Clear.render(area, buf);
            Paragraph::new(text)
                .block(block)
                .centered()
                .wrap(Wrap { trim: true })
                .render(area, buf);
        }
    }
}
