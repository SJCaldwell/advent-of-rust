use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn messy_string_to_integer(input: &str) -> i32 {
    let mut first_digit = 0;
    let mut second_digit = 0;
    for (_i, c) in input.chars().enumerate() {
        if c.is_digit(10) {
            if first_digit == 0 {
                first_digit = c.to_digit(10).unwrap() as i32;
            } else {
                second_digit = c.to_digit(10).unwrap() as i32;
            }
        }
    }
    if second_digit == 0 {
        second_digit = first_digit;
    }
    println!("First digit: {}, second digit: {}", first_digit, second_digit);
    let sum = first_digit * 10 + second_digit;
    println!("Sum: {}", sum);
    sum
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let path =Path::new(&args[1]);

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open file {} because: {}", path.display(), why),
    };

    let reader = io::BufReader::new(file);
    let mut sum = 0;

    for line in reader.lines(){
        match line{
            Ok(line) => {
                sum += messy_string_to_integer(&line);
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    println!("The total sum is {}", sum);
}
