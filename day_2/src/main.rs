extern crate regex;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;
use std::error::Error;

use regex::Regex;

fn task_one() -> Result<usize, Box<dyn Error>> {
    let file = BufReader::new(File::open("input")?).lines();
    let regex = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<req>\w): (?P<password>\w+)").unwrap();
    let count = file.fold(0, |i, line| {
        let line_string = line.unwrap();
        println!("trying: {}", line_string);
        let values = regex.captures(&line_string).unwrap();
        let requirement = values.name("req").unwrap().as_str().chars().nth(0).unwrap();
        let char_count = values.name("password").unwrap().as_str().chars().fold(0, |count, letter| {
            count + (letter == requirement) as usize
        });
        let min = usize::from_str(values.name("min").unwrap().as_str()).unwrap();
        let max = usize::from_str(values.name("max").unwrap().as_str()).unwrap();
        i + (min <= char_count && char_count <= max) as usize
    });
    Ok(count)
}

fn task_two() -> Result<usize, Box<dyn Error>> {
    let file = BufReader::new(File::open("input")?).lines();
    let regex = Regex::new(r"(?P<lower>\d+)-(?P<upper>\d+) (?P<req>\w): (?P<password>\w+)").unwrap();
    let count = file.fold(0, |i, line| {
        let line_string = line.unwrap();
        println!("trying: {}", line_string);
        let values = regex.captures(&line_string).unwrap();
        let requirement = values.name("req").unwrap().as_str().chars().nth(0).unwrap();
        let password = values.name("password").unwrap().as_str();
        let lower = usize::from_str(values.name("lower").unwrap().as_str()).unwrap();
        let upper = usize::from_str(values.name("upper").unwrap().as_str()).unwrap();
        i + ((&password.chars().nth(lower-1).unwrap() == &requirement) ^ (&password.chars().nth(upper-1).unwrap() == &requirement)) as usize
    });
    Ok(count)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Task 1: {}", task_one()?);
    println!("Task 2: {}", task_two()?);
    Ok(())
}
