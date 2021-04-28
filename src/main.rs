#[macro_use]
extern crate clap;
extern crate ctrlc;
extern crate serde;
extern crate serde_yaml;

use notify_rust::Notification;
use std::{ process, thread, time };
use std::collections::BTreeMap;
use std::io::Write;

const TASKS_FILE: &str = "tasks.yaml";

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

    if let Some(ref matches) = matches.subcommand_matches("add") {
        let index = matches
            .value_of("INDEX")
            .and_then(|value| { Some(value.parse().unwrap())});
        let task = matches
            .values_of("TASK")
            .unwrap()
            .collect::<Vec<&str>>()
            .join(" ");

        add_task(&task, index).expect("Failed to add task");
    } else if let Some(ref matches) = matches.subcommand_matches("remove") {
        let index = matches
            .value_of("INDEX")
            .unwrap()
            .parse()
            .unwrap();

        remove_task(index).expect("Failed to remove task");
    } else if let Some(ref matches) = matches.subcommand_matches("start") {
        let duration: u64 = matches
            .value_of("DURATION")
            .unwrap_or("30")
            .parse()
            .unwrap();

            start_notifications(duration).expect("Failed to start notifications");
    } else {
        list_tasks().expect("Failed to lists tasks");
    }
}

fn add_task(task: &str, index: Option<usize>) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut tasks = read_tasks()?;
    let task = String::from(task);

    println!("Adding task: '{}'...", task);

    if let Some(index) = index {
        tasks.insert(index, task)
    } else {
        tasks.push(task);
    }
    write_tasks(&tasks)?;
    println!("Done");
    Ok(())
}

fn list_tasks() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let tasks = read_tasks()?;

    println!("Tasks:");
    for (i, task) in tasks.iter().enumerate() {
        println!("{}.\t{}", i, task);
    }
    Ok(())
}

fn read_tasks() -> Result<Vec<String>, Box<dyn std::error::Error + 'static>> {
    let f = std::fs::File::open(TASKS_FILE)?;
    let data: serde_yaml::Value = serde_yaml::from_reader(f)?;

    Ok(data["tasks"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|s| { String::from(s.as_str().unwrap()) })
        .collect()
    )
}

fn remove_task(index: usize) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut tasks = read_tasks()?;

    if index < tasks.len() {
        let task = &tasks[index];

        println!("Removing task: '{}'...", task);
        tasks.remove(index);
        write_tasks(&tasks)?;
        println!("Done");
    } else {
        println!("No task with index: {}", index);
    }

    Ok(())
}

fn start_notifications(duration: u64) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let timer = time::Duration::from_secs(duration * 60);
    let tasks = read_tasks()?;
    let content = format!("{}", tasks.join("\n"));

    loop {
        match Notification::new()
            .summary("Creak - Tasks")
            .body(&content)
            .appname("creak")
            .timeout(0)
            .show() {
                _ => ()
            }

        thread::sleep(timer);
    }
}

fn write_tasks(tasks: &Vec<String>) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut map = BTreeMap::new();

    map.insert("tasks".to_string(), tasks);

    let s = serde_yaml::to_string(&map)?;
    let mut f = std::fs::File::create(TASKS_FILE)?;

    f.write_all(s.as_bytes())?;
    Ok(())
}
