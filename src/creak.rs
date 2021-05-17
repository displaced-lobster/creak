extern crate dirs;
extern crate serde;
extern crate serde_yaml;

use notify_rust::Notification;
use std::{ thread, time };
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{ Path, PathBuf };

const TASKS_FILE: &str = ".creak/tasks.yaml";
const TEN_MINUTES: u64 = 10 * 60;

pub struct Creak {
    path: PathBuf,
    tasks: Vec<String>,
}

impl Creak {
    pub fn add_task(&mut self, task: &str, index: Option<usize>) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let task = String::from(task);

        println!("Adding task: {[bold]}...", task);

        if let Some(index) = index {
            self.tasks.insert(index, task)
        } else {
            self.tasks.push(task);
        }
        self.write()?;
        println!("Done");
        self.list_tasks()
    }

    pub fn init() -> Result<Creak, Box<dyn std::error::Error + 'static>> {
        let path = task_file_path();
        let tasks = read_tasks(&path)?;

        Ok(Creak{ path, tasks })
    }

    pub fn list_tasks(&self) -> Result<(), Box<dyn std::error::Error + 'static>> {
        println!("{$underline}Tasks{/$}:");
        for (i, task) in self.tasks.iter().enumerate() {
            println!("{[dimmed]}.\t{[bold]}", i, task);
        }
        Ok(())
    }

    pub fn print_config(&self) {
        println!("{$underline}Configuration{/$}:");
        println!("\tTask file: {[bold]}", self.path.as_path().display().to_string());
    }

    pub fn remove_task(&mut self, index: usize) -> Result<(), Box<dyn std::error::Error + 'static>> {
        if index < self.tasks.len() {
            let task = &self.tasks[index];

            println!("Removing task: {[bold]}...", task);
            self.tasks.remove(index);
            self.write()?;
            println!("Done");
        } else {
            println!("No task with index: {}", index);
        }

        self.list_tasks()
    }

    pub fn start_notifications(&self, duration: u64) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let mut complete_count = 0;

        println!("{$bold}Starting notifications{/$}");

        loop {
            let complete_content = format!("<s>{}</s>", &self.tasks[0..complete_count].join("\n"));
            let incomplete_content = format!("{}", &self.tasks[complete_count..].join("\n"));
            let content = match complete_count > 0 {
                false => format!("{}", incomplete_content),
                true => format!("{}\n{}", complete_content, incomplete_content)
            };
            let mut timer = time::Duration::from_secs(duration * 60);

            Notification::new()
                .summary("Creak - Tasks")
                .body(&content)
                .appname("creak")
                .timeout(0)
                .action("complete", "Complete")
                .action("snooze", "Snooze")
                .show()?
                .wait_for_action(|action| match action {
                    "complete" => complete_count += 1,
                    "snooze" => timer = time::Duration::from_secs(TEN_MINUTES),
                    "__closed" => (),
                    _ => ()
                });

            thread::sleep(timer);
        }
    }

    fn write(&self) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let mut map = BTreeMap::new();

        map.insert("tasks".to_string(), &self.tasks);

        let s = serde_yaml::to_string(&map)?;
        let mut f = std::fs::File::create(&self.path)?;

        f.write_all(s.as_bytes())?;
        Ok(())
    }
}

fn read_tasks(path: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error + 'static>> {
    let f = std::fs::File::open(path)?;
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
