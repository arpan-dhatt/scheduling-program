use std::io::prelude::*;
use std::io::BufReader;
use std::fs::{File, read};
use std::collections::HashMap;

fn main() {
    let mut students_file_path = String::new();
    std::io::stdin().read_line(&mut students_file_path).expect("could not read students file path!");
    let students_data = read_students(&students_file_path);

    let mut classes_and_blocks_file_path = String::new();
    std::io::stdin().read_line(&mut classes_and_blocks_file_path).expect("could not read classes and blocks file path!");
    let classes_and_blocks_data = read_classes_and_blocks(&classes_and_blocks_file_path);



}

fn read_students(file_path: &str) -> Vec<(String,Vec<String>)> {
    let f = File::open(file_path).expect("could not open students file!");
    let mut reader = BufReader::new(f);
    let mut out = Vec::new();
    for n in reader.lines() {
        n.expect("could not read number of lines in students file!");
        let mut line = String::new();
        reader.read_line(&mut line);
        let split: Vec<&str> = line.split(",").collect();
        for (i, s) in split.iter().enumerate() {
            if i == 0 { //on student name
                out.push((String::from(s), Vec::new()));
            } else { //on a block they are planning to take
                out[out.len()-1].1.push(String::from(s))
            }
        }
    }
    return out;
}

fn read_classes_and_blocks(file_path: &str) -> Vec<(String,Vec<usize>)> {
    let f = File::open(file_path).expect("could not open classes and blocks file!");
    let mut reader = BufReader::new(f);
    let mut out = Vec::new();
    for n in reader.lines() {
        n.expect("could not read number of lines in classes and blocks file!");
        let mut line = String::new();
        reader.read_line(&mut line);
        let split: Vec<&str> = line.split(",").collect();
        for (i, s) in split.iter().enumerate() {
            if i == 0 { //on class name
                out.push((String::from(s), Vec::new()));
            } else { //on a block the class is available
                out[out.len()-1].1.push(
                    s.parse().expect("invalid block number in classes and blocks file!")
                );
            }
        }
    }
    return out;
}

fn get_classes_to_blocks_map(classes_and_blocks: &Vec<(String,Vec<usize>)>) -> (HashMap<String,[bool; 8]>) {
    let mut out = HashMap::new();
    for (class, blocks) in classes_and_blocks {
        let mut bools = [false; 8];
        for block in blocks {
            bools[block-1] = true;
        }
        out.insert(class.clone(),bools);
    }
    return out;
}

fn get_blocks_to_classes_map(classes_and_blocks: &Vec<(String,Vec<usize>)>) -> [Vec<String>; 8] {
    let mut out = [Vec::new(); 8];
    for (class, blocks) in classes_and_blocks {
        for block in blocks {
            out[block-1].push(class);
        }
    }
    return out;
}

fn get_all_valid_classes(blocks_to_classes: &[Vec<String>; 8], output: &mut Vec<[String; 8]>, input: &[String; 8], mut work: [String; 8], block: usize) {
    for class in blocks_to_classes[block] {
        if !work.contains(&class) && input.contains(&class) {
            work[block].push_str(&class);
            if block == 7 {
                output.push(work.clone());
                return;
            }
            get_all_valid_classes(blocks_to_classes, output, input, work.clone(), block+1);
        }
    }
}