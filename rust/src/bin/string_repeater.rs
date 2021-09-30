extern crate rust;

use std::io::stdin;
use rust::repeated_string::repeated_string;

fn main() {
    let mut string_to_repeat = String::new();

    stdin()
        .read_line(&mut string_to_repeat)
        .ok()
        .expect("Failed to read line");

    let mut chars_to_consider = String::new();

    stdin()
        .read_line(&mut chars_to_consider)
        .ok()
        .expect("Failed to read line");

    repeated_string(
        &string_to_repeat.trim(),
        chars_to_consider.trim().parse::<i32>().unwrap() as usize,
    );
}
