#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn aoc_03() {
    let mut priorities_sum = 0;
    
    let file = File::open("data/03-rucksacks.txt").expect("Could not open file.");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line: String = line.unwrap();
        let half_length = line.len() / 2;
        let first_part = &line[0..half_length];
        let second_part = &line[half_length..];

        match first_part.chars().find(|c| second_part.contains(*c)) {
            Some(c) => {
                let priority = if c.is_lowercase() { (c as u8) - 96 } else { (c as u8) - 38 };
                println!("Doubled item {} with priority {}.", c, priority);
                priorities_sum += priority;
            },
            None => println!("No doubled items found."),
        };
    }

    println!("The sum of priorities is {}", priorities_sum);
}
