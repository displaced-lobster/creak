#[macro_use]
extern crate clap;
extern crate ctrlc;

use notify_rust::Notification;
use std::{ thread, time };

fn main() {
    let matches = clap_app!(creak =>
        (version: "0.4.0")
        (author: "Richard Mills <scripts.richard@gmail.com>")
        (about: "The creaky wheel gets the oil")
        (@arg DURATION: -d --duration +takes_value "Set the reminder duration in minutes")
    ).get_matches();
    let duration:u64 = matches
        .value_of("DURATION")
        .unwrap_or("30")
        .parse()
        .unwrap();
    let timer = time::Duration::from_secs(duration * 60);

    println!("Starting reminder to run every {} minutes", duration);

    loop {
        thread::sleep(timer);

        match Notification::new()
            .summary("Creak")
            .body("Do something!")
            .appname("creak")
            .timeout(0)
            .show() {
                _ => ()
            }
    }
}
