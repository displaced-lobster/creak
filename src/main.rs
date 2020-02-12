#[macro_use]
extern crate clap;
extern crate termion;

use notify_rust::Notification;
use std::io;
use std::io::Write;
use std::{thread, time};

fn main() {
    let matches = clap_app!(creak =>
        (version: "0.1.0")
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

    clear_and_reset();
    println!("Starting standing routine to run in {} minute intervals", duration);

    loop {
        for _ in 0..60 {
            thread::sleep(timer);
            print!(".");
            io::stdout().flush().expect("Failed to flush");
        }

        let mut s = String::new();
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

        clear_and_reset();
        print!("Stand up! Hit ENTER to continue");

        io::stdout().flush().expect("Failed to flush");
        io::stdin().read_line(&mut s).expect("Failed to read user input");

        clear_and_reset();
        println!("{} elapsed after timer expired", time_string(now.elapsed()));
        println!("{} minute timer reset", duration);
    }

}

fn time_string(t: time::Duration) -> std::string::String {
    let t = t.as_secs();
    let minutes = t / 60;
    let seconds = t % 60;

    format!("{} minutes {} seconds", minutes, seconds)
}

fn clear_and_reset() {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
}
