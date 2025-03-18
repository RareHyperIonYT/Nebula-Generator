mod app;
mod password;
mod ui;

use color_eyre::Result;
use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let res = App::new().run(terminal);
    ratatui::restore();
    res
}