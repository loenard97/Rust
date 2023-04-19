#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

struct FSFile {
    name: String,
    size: u32,
}

impl FSFile {
    fn from_string(cmd: String) -> Self {
        FSFile { name: "".to_string(), size: 0 }
    }
}

struct FSDir {
    name: String,
    dirs: Vec<Self>,
    files: Vec<FSFile>,
}

impl FSDir {
    fn from_string(cmd: String) -> Self {
        FSDir { name: "".to_string(), dirs: vec![], files: vec![] }
    }

    fn size(&self) -> u32 {
        0
    }
}

struct Filesystem {
    cur_path: Vec<String>,
    root_dir: FSDir,
}

impl Filesystem {
    fn get_cur_dir(&self) -> &FSDir {
        let mut cur_dir = &self.root_dir;
        for dir in self.cur_path.iter() {
            cur_dir = cur_dir.dirs.iter()
                .find(|fsdir| fsdir.name == *dir)
                .unwrap();
        }
        cur_dir
    }

    fn cd(&mut self, cmd: &String) {
        if cmd == &"$ cd /".to_string() {
            self.cur_path.clear();
        } else if cmd == &"$ cd ..".to_string() {
            self.cur_path.pop();
        } else if cmd.starts_with("$ cd ") {
            self.cur_path.push(cmd[3..cmd.len()].to_string());
        }
    }

    fn add_file(dir: &mut FSFile, cmd: String) {

    }

    fn add_dir(dir: &mut FSDir, cmd: String) {

    }

    fn from_file(file_path: &str) -> Filesystem {

        let file = File::open(file_path).expect("Could not read file.");
        let reader = BufReader::new(file);
    
        let mut ret_fs = Filesystem { 
            cur_path: vec![], 
            root_dir: FSDir { name: "/".to_string(), dirs: vec![], files: vec![] } 
        };

        for line in reader.lines() {
            let line = line.unwrap();   // remove whitespace
    
            if line.starts_with("$ cd ") {
                ret_fs.cd(&line);
            } else if line.starts_with("& ls") {

            }
    
        }

        ret_fs
    }

    fn find_large_dirs(&self) -> Vec<FSDir> {
        vec![]
    }
}

pub fn aoc_07() {
    let file_path = "data/07-terminal.txt";
    let fs = Filesystem::from_file(file_path);

    let large_dirs = fs.find_large_dirs();
}
