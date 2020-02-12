use std::io;
use std::io::Write;
use std::{thread, time};
use notify_rust::Notification;

#[macro_use]
extern crate clap;

fn main() {
    let duration = 1;
    let matches = clap_app!(creak =>
        (version: "0.1.0")
        (author: "Richard Mills <scripts.richard@gmail.com>")
        (about: "A prolonged sitter's best friend")
        (@arg verbose: -d "Set the level of verbosity")
    ).get_matches();
    let _verbose = matches.is_present("verbose");
    let timer = time::Duration::from_secs(duration);

    print!("\x1B[2J");
    println!("Starting standing routine to run every 15 minutes");

    loop {
        for _ in 0..60 {
            thread::sleep(timer);
            print!(".");
            io::stdout().flush().expect("Failed to flush");
        }

        let mut s = String::new();
        let now = time::Instant::now();

        Notification::new()
            .summary("Creak")
            .body("Time to stand up.")
            .show()
            .unwrap();
        print!("\nHave you stood up?");

        io::stdout().flush().expect("Failed to flush");
        io::stdin().read_line(&mut s).expect("Failed to read user input");

        print!("\x1B[2J");
        println!("{:?} elapsed after timer expired", now.elapsed());
        println!("Starting next 15 minute timer");
    }

}
