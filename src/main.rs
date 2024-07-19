use std::io;

mod app;
mod connector;
mod drivers;
mod editor;
mod tui;

use app::App;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;

    let app_result = App::default().run(&mut terminal);

    tui::restore()?;

    app_result
}
