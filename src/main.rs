#[macro_use]
extern crate clap;
extern crate ctrlc;

use notify_rust::Notification;
use std::{process, thread, time};
use std::sync::{Arc, Mutex};

use creak::time_string;
use creak::terminal::Terminal;

fn main() {
    let matches = clap_app!(creak =>
        (version: "0.2.0")
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
    let history: Arc<Mutex<Vec<time::Duration>>> = Arc::new(Mutex::new(Vec::new()));

    let shared_history = history.clone();
    ctrlc::set_handler(move || {
        for v in shared_history.lock().unwrap().iter() {
            println!("{:?}", v);
        }
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

        let elapsed = now.elapsed();

        history.lock().unwrap().push(elapsed);

        println!("{} elapsed after timer expired", time_string(elapsed));
        println!("{} minute timer reset", duration);
    }
}
