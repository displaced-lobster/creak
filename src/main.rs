#[macro_use]
extern crate clap;
extern crate ctrlc;

use std::process;

mod creak;

fn main() {
    ctrlc::set_handler(move || {
        println!("\nStopping notifications");
        process::exit(0);
    }).expect("Failed to set Ctrl-C handler");

    let matches = clap_app!(creak =>
        (version: "0.4.0")
        (author: "Richard Mills <scripts.richard@gmail.com>")
        (about: "The creaky wheel gets the oil")
        (@subcommand add =>
            (about: "add a task")
            (version: "0.1")
            (@arg TASK: +multiple +required "Task to be added")
            (@arg INDEX: -i --index +takes_value "Which index to add the task")
        )
        (@subcommand list =>
            (about: "list tasks")
            (version: "0.1")
        )
        (@subcommand remove =>
            (about: "remove task by index")
            (version: "0.1")
            (@arg INDEX: +required "Task index to be removed")
        )
        (@subcommand start =>
            (about: "start notifications")
            (version: "0.1")
            (@arg DURATION: -d --duration +takes_value "Set the reminder duration in minutes")
        )
    ).get_matches();

    let mut cr = creak::Creak::init().unwrap();

    if let Some(ref matches) = matches.subcommand_matches("add") {
        let index = matches
            .value_of("INDEX")
            .and_then(|value| { Some(value.parse().unwrap())});
        let task = matches
            .values_of("TASK")
            .unwrap()
            .collect::<Vec<&str>>()
            .join(" ");

        cr.add_task(&task, index).expect("Failed to add task");
    } else if let Some(ref matches) = matches.subcommand_matches("remove") {
        let index = matches
            .value_of("INDEX")
            .unwrap()
            .parse()
            .unwrap();

        cr.remove_task(index).expect("Failed to remove task");
    } else if let Some(ref matches) = matches.subcommand_matches("start") {
        let duration: u64 = matches
            .value_of("DURATION")
            .unwrap_or("30")
            .parse()
            .unwrap();

            cr.start_notifications(duration).expect("Failed to start notifications");
    } else {
        cr.list_tasks().expect("Failed to lists tasks");
    }
}
