#![allow(dead_code)]

use std::fs;

pub fn aoc_06() {
    let input = fs::read_to_string("data/06-stream.txt").expect("Could not read file");

    let max_index = input.len() - 4;
    let mut answer: usize = 0;

    for index in 0..=max_index {
        let part = &input[index..index+4];

        answer = index + 4;

        if part.chars()
            .map(|c| part.matches(c).count())
            .max()
            .unwrap_or(0)
            == 1 
            { break; }
    }

    println!("The sequence starts at index {}.", answer);
}
