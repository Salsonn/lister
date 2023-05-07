use std::env;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
    event,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let self_path = &args[0];
    let query = &args[1];
    let file_path = &args[2];

    println!("Running from: {}", self_path);
    println!("Searching for {}", query);
    println!("In file {}", file_path);
}