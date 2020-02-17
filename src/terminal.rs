extern crate termion;

use std::io;
use std::io::Write;
use termion::color;

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

    pub fn generate_chart(values: Vec<u64>, threshold: u64) {
        let (_height, width) = termion::terminal_size().expect("Failed to get terminal size");
        let mut max = 0;

        for v in &values {
            if v > &max {
                max = *v;
            }
        }

        for v in values {
            let n = (((v as f64) / (max as f64) * (width as f64)) as usize) * 2;
            let c;

            if v > threshold {
                c = format!("{}", color::Fg(color::Red));
            } else {
                c = format!("{}", color::Fg(color::Green))
            }

            println!("{}{}", c, "●".repeat(n));
        }
    }

    pub fn barchart(values: Vec<u64>, threshold: u64) {
        let (height, _width) = termion::terminal_size().expect("Failed to get terminal size");
        let mut max = 0;

        for v in &values {
            if *v > max {
                max = *v;
            }
        }

        let scale = (max as f64) / (height as f64);
        let new_values: Vec<u64> = values.iter().map(|v| ((*v as f64) * scale) as u64).collect();
        let height = height as u64;
        let threshold = ((threshold as f64) * scale) as u64;

        for y in 0..height {
            for v in &new_values {
                let c;
                if *v > threshold {
                    c = format!("{}", color::Fg(color::Red));
                } else {
                    c = format!("{}", color::Fg(color::Green))
                }

                if *v >= height - y {
                    print!("{}●", c);
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
