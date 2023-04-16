#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn aoc_01() {
    let mut calories: Vec<i32> = vec![0];

    let file = File::open("data/01-calories.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line: String = line.unwrap().split_whitespace().collect();
        match line.parse::<i32>() {
            Ok(val) => *calories.last_mut().unwrap() += val,
            Err(_) => calories.push(0),
        };
    }

    let max_value = calories.iter().max().unwrap();
    let max_index = calories.iter().position(|x| x == max_value).unwrap();

    println!("List of all calories: {:?}", calories);
    println!("The most calories has elf {} with {} calories.", max_index + 1, max_value);
}
