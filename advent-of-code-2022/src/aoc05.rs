#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

struct CargoMove {
    n_moves: u16,
    from: u16,
    to: u16,
}

impl CargoMove {
    fn from_file(file_path: &str) -> Vec<Self> {
        let mut ret: Vec<Self> = vec![];

        let file = File::open(file_path).expect("Could not open file.");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("move") {
                let splits: Vec<&str> = line.split_whitespace().collect();
                let splits: Vec<u16> = splits.iter().map(|c| c.parse::<u16>().unwrap_or(0)).collect();  // dumb
                ret.push(CargoMove { n_moves: splits[1], from: splits[3], to: splits[5] });
            }
        }
        ret
    }
}

struct CargoBay {
    bay: [VecDeque<char>; 3],
}

impl CargoBay {
    fn from_file(file_path: &str) -> Self {
        let mut first: VecDeque<char> = VecDeque::from([]);
        let mut second: VecDeque<char> = VecDeque::from([]);
        let mut third: VecDeque<char> = VecDeque::from([]);

        let file = File::open(file_path).expect("Could not open file.");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();

            if line.is_empty() { break; }

            let c1 = &line[1..2].parse::<char>().unwrap();
            let c2 = &line[5..6].parse::<char>().unwrap();
            let c3 = &line[9..10].parse::<char>().unwrap();

            if c1.is_alphabetic() && !c1.is_whitespace() { first.push_front(*c1) };
            if c2.is_alphabetic() && !c2.is_whitespace() { second.push_front(*c2) };
            if c3.is_alphabetic() && !c3.is_whitespace() { third.push_front(*c3) };
        }

        CargoBay { bay: [first, second, third] }
    }

    fn move_cargo(&mut self, move_operation: &CargoMove) {
        for _ in 0..move_operation.n_moves {
            let index_from = (move_operation.from - 1) as usize;
            let index_to = (move_operation.to - 1) as usize;
    
            if let Some(last_char) = self.bay[index_from].pop_back() {
                self.bay[index_to].push_back(last_char);
            }
        }
    }

    fn bay_size (&self) -> usize {
        self.bay.len()
    }

    fn max_bay_load(&self) -> usize {
        self.bay.iter()
            .map(|cur_vec| cur_vec.len())
            .max()
            .unwrap_or(0)
    }

    fn as_string(&self) -> String {
        let mut ret = String::new();

        for cur_index in (0..self.max_bay_load()).rev() {
            for cur_bay in 0..self.bay_size() {
                if cur_index < self.bay[cur_bay].len() {
                    ret.push_str("[");
                    ret.push(self.bay[cur_bay][cur_index]);
                    ret.push(']');
                } else {
                    ret.push_str("   ");
                }
            }
            ret.push('\n');
        }

        for cur_bay in 0..self.bay_size() {
            ret.push(' ');
            ret.push_str(&cur_bay.to_string());
            ret.push(' ');
        }

        ret
    }

    fn top_as_string(&self) -> String {
        let mut ret = String::new();

        for elem in self.bay.iter() {
            let c = elem.back().unwrap();
            ret.push(*c);
        }
        ret
    }
}

pub fn aoc_05() {
    // load initial state from file
    let file_path = "data/05-cargo.txt";
    let mut cargo_bay = CargoBay::from_file(file_path);
    let move_operations = CargoMove::from_file(file_path);
    
    println!("\nInitial CargoBay state:");
    println!("{}", cargo_bay.as_string());

    // apply move operations
    for move_op in move_operations.iter() {
        cargo_bay.move_cargo(&move_op);
    }

    // print final state
    println!("\nFinal CargoBay state:");
    println!("{}", cargo_bay.as_string());
    println!("\nTop most crates:");
    println!("{}", cargo_bay.top_as_string());
}
