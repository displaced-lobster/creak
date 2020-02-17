#[macro_use]
extern crate clap;
extern crate ctrlc;

use notify_rust::Notification;
use std::{process, thread, time};
use std::sync::{Arc, Mutex};
use termion::{color, style};

use creak::time_string;
use creak::terminal::Terminal;

const THRESHOLD: u64 = 120;

fn main() {
    let matches = clap_app!(creak =>
        (version: "0.3.0")
        (author: "Richard Mills <scripts.richard@gmail.com>")
        (about: "A prolonged sitter's best friend")
        (@arg quiet: -q "Stiffle output")
        (@arg DURATION: -d --duration +takes_value "Set the sitting duration in minutes")
    ).get_matches();
    let quiet = matches.is_present("quiet");
    let duration = matches
                    .value_of("DURATION")
                    .unwrap_or("15")
                    .parse()
                    .unwrap();
    let timer = time::Duration::from_secs(duration);
    let history: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(Vec::new()));
    let ctrlc_history = history.clone();

    ctrlc::set_handler(move || {
        let values = ctrlc_history.lock().unwrap().iter().map(|&e| e).collect();

        Terminal::clear_and_reset();
        Terminal::barchart(values, THRESHOLD);
        process::exit(0);
    }).expect("Failed to set Ctrl-C handler");

    Terminal::clear_and_reset();
    println!("Starting standing routine to run in {} minute intervals", duration);

    loop {
        for _ in 0..60 {
            thread::sleep(timer);
            Terminal::print_and_flush(".");
        }

        let now = time::Instant::now();

        if !quiet {
            match Notification::new()
                .summary("Creak")
                .body("Time to stand up.")
                .appname("creak")
                .timeout(0)
                .show() {
                    _ => ()
                }
        }

        Terminal::clear_and_reset();
        Terminal::print_and_flush("Stand up! Hit ENTER to continue");
        Terminal::wait_input();
        Terminal::clear_and_reset();

        let elapsed = now.elapsed().as_secs();

        history.lock().unwrap().push(elapsed);

        let c;

        if elapsed > THRESHOLD {
            c = format!("{}", color::Fg(color::Red));
        } else {
            c = format!("{}", color::Fg(color::Green))
        }

        print!("{}{}{}{} ", c, style::Bold, time_string(elapsed), style::Reset);
        println!("elapsed after timer expired");
        println!("{} minute timer reset", duration);
    }
}
