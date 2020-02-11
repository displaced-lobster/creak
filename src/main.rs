use std::io;
use std::io::Write;
use std::{thread, time};

#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(creak =>
        (version: "0.1.0")
        (author: "Richard Mills <scripts.richard@gmail.com>")
        (about: "A prolonged sitter's best friend")
        (@arg verbose: -d "Set the level of verbosity")
    ).get_matches();
    let _verbose = matches.is_present("verbose");
    let duration = time::Duration::from_secs(15);

    println!("Starting standing routine to run every 15 minutes");

    loop {
        for _ in 0..60 {
            thread::sleep(duration);
            print!(".");
            io::stdout().flush().expect("Failed to flush");
        }

        let mut s = String::new();
        let now = time::Instant::now();
        print!("\nHave you stood up? (Y/y) ");

        loop {
            io::stdout().flush().expect("Failed to flush");
            io::stdin().read_line(&mut s).expect("Failed to read user input");

            let c = s.chars().next().unwrap();

            if c == 'y' || c == 'Y' || c == '\n' {
                break;
            }
            print!("Invalid input (Y/y): ");
        }
        print!("\x1B[2J");
        println!("{:?} elapsed after timer expired", now.elapsed());
        println!("Starting next 15 minute timer");
    }

}
