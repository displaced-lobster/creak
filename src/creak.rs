extern crate dirs;
extern crate serde;
extern crate serde_yaml;

use notify_rust::Notification;
use std::{ thread, time };
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{ Path, PathBuf };

const TASKS_FILE: &str = ".creak/tasks.yaml";

pub struct Creak {
    tasks: Vec<String>
}

impl Creak {
    pub fn init() -> Result<Creak, Box<dyn std::error::Error + 'static>> {
        let tasks = read_tasks()?;

        Ok(Creak{ tasks })
    }

    pub fn add_task(&mut self, task: &str, index: Option<usize>) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let task = String::from(task);

        println!("Adding task: '{}'...", task);

        if let Some(index) = index {
            self.tasks.insert(index, task)
        } else {
            self.tasks.push(task);
        }
        self.write()?;
        println!("Done");
        self.list_tasks()
    }

    pub fn list_tasks(&self) -> Result<(), Box<dyn std::error::Error + 'static>> {
        println!("Tasks:");
        for (i, task) in self.tasks.iter().enumerate() {
            println!("{}.\t{}", i, task);
        }
        Ok(())
    }

    pub fn remove_task(&mut self, index: usize) -> Result<(), Box<dyn std::error::Error + 'static>> {
        if index < self.tasks.len() {
            let task = &self.tasks[index];

            println!("Removing task: '{}'...", task);
            self.tasks.remove(index);
            self.write()?;
            println!("Done");
        } else {
            println!("No task with index: {}", index);
        }

        self.list_tasks()
    }

    pub fn start_notifications(&self, duration: u64) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let timer = time::Duration::from_secs(duration * 60);
        let content = format!("{}", self.tasks.join("\n"));

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

    fn write(&self) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let file_path = task_file_path();
        let mut map = BTreeMap::new();

        map.insert("tasks".to_string(), &self.tasks);

        let s = serde_yaml::to_string(&map)?;
        let mut f = std::fs::File::create(file_path)?;

        f.write_all(s.as_bytes())?;
        Ok(())
    }
}

fn read_tasks() -> Result<Vec<String>, Box<dyn std::error::Error + 'static>> {
    let file_path = task_file_path();
    let f = std::fs::File::open(file_path)?;
    let data: serde_yaml::Value = serde_yaml::from_reader(f)?;

    Ok(data["tasks"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|s| { String::from(s.as_str().unwrap()) })
        .collect()
    )
}

fn task_file_path() -> PathBuf {
    let home_dir = dirs::home_dir().unwrap();

    Path::new(&home_dir).join(TASKS_FILE)
}
