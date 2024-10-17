use std::io::{self, stdout};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    terminal::{self, Clear, ClearType},
};

pub fn hide_cursor() {
    execute!(io::stdout(), Hide).expect("Could not hide cursor");
}

pub fn show_cursor() {
    execute!(io::stdout(), Show).expect("Could not show cursor");
}

pub fn move_cursor(col: u16, line: u16) {
    execute!(io::stdout(), MoveTo(col, line)).expect("Could not move cursor")
}

pub fn clear_terminal() {
    execute!(stdout(), Clear(ClearType::All)).expect("Could not clear terminal")
}

pub fn get_terminal_size() -> (u16, u16) {
    let (w, h) = terminal::size().expect("Could not retrieve terminal size");
    (w, h)
}
