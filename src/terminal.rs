extern crate termion;

use std::io;
use std::io::Write;

pub struct Terminal();

impl Terminal {
    pub fn print_and_flush(line: &str) {
        print!("{}", line);
        io::stdout().flush().expect("Failed to flush");
    }

    pub fn clear_and_reset() {
        print!("{}{}",
               termion::clear::All,
               termion::cursor::Goto(1, 1));
    }

    pub fn wait_input() {
        let mut s = String::new();

        io::stdin()
            .read_line(&mut s)
            .expect("Failed to read user input");
    }
}
