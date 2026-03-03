mod animations;
mod app;
mod bullet;
mod enemy;
mod player;
mod traits;
// mod tui;
mod utils;
mod wall;
use std::io::{self, stdout};

use app::App;
use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::{enemy::Enemy, wall::Wall};

fn main() -> io::Result<()> {
    let mut lvl: u16 = 1;
    loop {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;
        let n_enemies = (lvl).clamp(1, 8);
        let n_walls = (n_enemies * 3 / 2).clamp(1, 9);
        let factor = (lvl % 5).clamp(1, 4);
        let div_factor = (lvl.saturating_add(3)) % 5;
        let mut app = App::new_random(&terminal, n_walls, n_enemies, factor, div_factor, lvl)?;
        let result = ratatui::run(|terminal| app.run(terminal));
        if !(app.restart && result.is_ok()) {
            return result;
        } else if app.restart && app.enemies.is_empty() {
            lvl = lvl.saturating_add(1);
        }
    }
}
