use std::time;

pub mod terminal;

pub fn time_string(t: time::Duration) -> std::string::String {
    let t = t.as_secs();
    let minutes = t / 60;
    let seconds = t % 60;

    format!("{} minutes {} seconds", minutes, seconds)
}
