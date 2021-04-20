#[macro_use]
extern crate clap;
extern crate ctrlc;
extern crate serde;
extern crate serde_yaml;

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

    let f = std::fs::File::open("tasks.yaml").unwrap();
    let data: serde_yaml::Value = serde_yaml::from_reader(f).unwrap();
    let tasks: Vec<String> = data["tasks"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|s| { String::from(s.as_str().unwrap()) })
        .collect();

    let content = format!("{}", tasks.join("\n"));

    loop {
        thread::sleep(timer);

        match Notification::new()
            .summary("Creak - TODO")
            .body(&content)
            .appname("creak")
            .timeout(0)
            .show() {
                _ => ()
            }
    }
}
