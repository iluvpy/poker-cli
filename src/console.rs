use std::{io::{self, Write}, time::{Duration}, thread::{sleep}, time, thread};

use crate::util::flush_stdout;


pub struct Input {
    pub number: Option<i32>,
    pub string: Option<String>
}

pub fn get_input(message: &str) -> Input {
    let text_input = get_str(message);
    Input{
        string: Some(text_input.clone()),
        number: match text_input.trim().parse::<i32>() {
            Ok(num) => Some(num),
            Err(_) => None
        }
    }
}

pub fn get_text(message: &str) -> String {
    String::from(get_str(message).trim())
}

pub fn get_number(message: &str) -> i32 {
    let input = get_input(message);
    match input.number.is_none() {
        false => input.number.unwrap(),
        true => {
            println!("please write a number!");
            get_number(message)
        }
    }
}

fn get_str(message: &str) -> String {
    print!("{}", message);
    let _ = io::stdout().flush();
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to get input");
    input
}

pub fn yes_no(message: &str) -> bool {
    let response = get_str(message);
    response.contains("y") || response.trim().is_empty()
}

pub fn endl(n: i32) {
    for _ in 0..n {
        print!("\n"); 
    }
} 

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn del_line() {
    print!("\r");
    flush_stdout();
}


pub fn dot_loading_animation(text: &str, finished_message: &str, iterations: i32) {
    for _ in 0..iterations {
        let dots = ["", ".", "..", "..."];
        for dot in dots {
            print!("\r{}{}   ", text, dot);
            flush_stdout();
            sleep(Duration::from_millis(500));
        }
    }
    println!("{}", finished_message);
}
