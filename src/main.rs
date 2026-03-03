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

fn main() -> io::Result<()> {
    let mut lvl: u16 = 1;
    let mut is_hard = false;
    loop {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;
        let n_enemies = (lvl).clamp(1, 9);
        let n_walls = (n_enemies * 3 / 2).clamp(1, 9);
        let factor = (lvl % 5).clamp(1, 4);
        let div_factor = (lvl.saturating_add(3)) % 5;
        let mut app = App::new_random(
            &terminal, n_walls, n_enemies, factor, div_factor, lvl, is_hard,
        )?;
        let result = ratatui::run(|terminal| app.run(terminal));
        if !app.is_restarting() || result.is_err() {
            return result;
        } else if app.is_restarting() && !app.is_lost() {
            lvl = lvl.saturating_add(1);
        }
        is_hard = app.is_hard();
    }
}
