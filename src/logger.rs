use colored::Colorize;

pub fn success(message: &str) {
    println!("[{}] {}", "+".green(), message);
}

pub fn info(message: &str) {
    println!("[{}] {}", "*".blue(), message);
}

pub fn error(message: &str) {
    println!("[{}] {}", "-".red(), message);
}

pub fn fatal(message: &str) {
    println!("[{}] {}", "!".on_red(), message);
}

pub fn warn(message: &str) {
    println!("[{}] {}", "!".yellow(), message);
}