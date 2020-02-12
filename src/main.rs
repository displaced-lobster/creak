use std::io;
use std::io::Write;
use std::{thread, time};
use notify_rust::Notification;

#[macro_use]
extern crate clap;

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

    print!("\x1B[2J");
    println!("Starting standing routine to run {} minute intervals", duration);

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
                .show() {
                    _ => ()
                }
        }

        print!("Stand up! Hit ENTER to continue");

        io::stdout().flush().expect("Failed to flush");
        io::stdin().read_line(&mut s).expect("Failed to read user input");

        print!("\x1B[2J");
        println!("{:?} elapsed after timer expired", now.elapsed());
        println!("{} minute timer reset", duration);
    }

}
