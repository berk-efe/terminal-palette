use std::io;

mod app;
mod widgets;

use crate::app::App;

#[macro_export]
macro_rules! margin {
    ($horizontal:expr, $vertical:expr) => {
        ratatui::layout::Margin {
            horizontal: $horizontal,
            vertical: $vertical,
        }
    };
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
