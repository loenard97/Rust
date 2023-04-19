#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn aoc_04() {
    let mut overlaps: u16 = 0;

    let file = File::open("data/04-assignements.txt").expect("Could not open file.");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let splits: Vec<&str> = line.split(|c| { c == ',' || c == '-' }).collect();
        let splits: Vec<u16> = splits.iter().map(|c| { c.parse::<u16>().unwrap() }).collect();

        if splits[0] <= splits[2] && splits[1] >= splits[3] || splits[0] >= splits[2] && splits[1] <= splits[3] {
            overlaps += 1;
        }
    }
    println!("There are a total of {} overlaps.", overlaps);
}
