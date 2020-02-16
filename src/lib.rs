pub mod terminal;

pub fn time_string(t: u64) -> std::string::String {
    let minutes = t / 60;
    let seconds = t % 60;

    format!("{} minutes {} seconds", minutes, seconds)
}
