use std::io::prelude::*;
use std::io::BufReader;
use std::fs::{File, read, read_to_string};
use std::collections::{HashMap, BTreeMap, HashSet};
use rand;
use rand::{random, thread_rng, Rng};
use rand::prelude::ThreadRng;

fn main() {
    let mut students_file_path = String::new();
    println!("enter students file path");
    std::io::stdin().read_line(&mut students_file_path).expect("could not read students file path!");
    let students_data = read_students(&students_file_path.trim());
    println!("{:?}", students_data);
    let mut classes_and_blocks_file_path = String::new();
    println!("enter classes and blocks file path");
    std::io::stdin().read_line(&mut classes_and_blocks_file_path).expect("could not read classes and blocks file path!");
    let classes_and_blocks_data = read_classes_and_blocks(&classes_and_blocks_file_path.trim());
    println!("{:?}", classes_and_blocks_data);

    let classes_to_blocks = get_classes_to_blocks_map(&classes_and_blocks_data);
    let blocks_to_classes = get_blocks_to_classes_map(&classes_and_blocks_data);

    let mut students_to_valid_schedules = BTreeMap::new();
    for (student, classes) in students_data {
        let mut valid_schedules = Vec::new();
        let mut work = [String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new()];
        get_all_valid_classes(&blocks_to_classes, &mut valid_schedules, &classes, work, 0);
        students_to_valid_schedules.insert(student.clone(),valid_schedules);
    }

    println!("valid schedules below:");
    for (student, schedules) in &students_to_valid_schedules {
        println!("{}({})", student,schedules.len());
        for schedule in schedules {
            println!("{:?}",schedule);
        }
        println!();
    }

    let mut max_permutations = 1;
    for schedule in students_to_valid_schedules.values() {
        max_permutations *= schedule.len();
    }

    println!("Maximum permutations: {}", max_permutations);
    println!("how many tries to make optimal schedule?");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("could not read number of tries!");
    let tries = buffer.trim().parse().expect("could not parse number of tries!");

    let mut index_scores = calc_scores(&students_to_valid_schedules, tries);
    index_scores.sort_by(|a,b|a.0.partial_cmp(&b.0).unwrap());
    let schedules_scores = convert_index_scores(&students_to_valid_schedules, index_scores);
    for (score, schedules) in schedules_scores {
        println!("Score: {}",score);
        for schedules in schedules {
            println!("{}:\t\t\t{:?}", schedules.0, schedules.1);
        }
    }
}

fn read_students(file_path: &str) -> Vec<(String,Vec<String>)> {
    let f = read_to_string(file_path).expect("could not read students file!");
    let mut out = Vec::new();
    for line in f.split("\n") {
        let split: Vec<&str> = line.split(",").collect();
        for (i, s) in split.iter().enumerate() {
            if i == 0 { //on student name
                out.push((String::from(*s), Vec::new()));
            } else { //on a block they are planning to take
                out.last_mut().unwrap().1.push(String::from(s.trim()))
            }
        }
    }
    return out;
}

fn read_classes_and_blocks(file_path: &str) -> Vec<(String,Vec<usize>)> {
    let f = read_to_string(file_path).expect("could not read students file!");
    let mut out = Vec::new();
    for line in f.split("\n") {
        let split: Vec<&str> = line.split(",").collect();
        for (i, s) in split.iter().enumerate() {
            if i == 0 { //on class name
                out.push((String::from(*s), Vec::new()));
            } else { //on a block the class is available
                out.last_mut().unwrap().1.push(
                    s.trim().parse().expect("invalid block number in classes and blocks file!")
                );
            }
        }
    }
    return out;
}

fn get_classes_to_blocks_map(classes_and_blocks: &Vec<(String,Vec<usize>)>) -> BTreeMap<String,[bool; 8]> {
    let mut out = BTreeMap::new();
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
    let mut out= [Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()];
    for (class, blocks) in classes_and_blocks {
        for block in blocks {
            out[block-1].push(class.clone());
        }
    }
    return out;
}

fn get_all_valid_classes(blocks_to_classes: &[Vec<String>; 8], output: &mut Vec<[String; 8]>, input: &Vec<String>, mut work: [String; 8], mut block: usize) {

    for class in &blocks_to_classes[block] {
        if !work.contains(&class) && input.contains(&class) {
            work[block] = class.clone();
            if block == 7 {
                output.push(copy_string_8_array(&work));
                return;
            }
            get_all_valid_classes(blocks_to_classes, output, input, copy_string_8_array(&work), block+1);
        }
    }
}

fn copy_string_8_array(array: &[String; 8]) -> [String; 8] {
    let mut out = [String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new()];
    for (i, e) in array.iter().enumerate() {
        out[i].push_str(e);
    }
    return out;
}

fn calc_scores(students_to_valid_schedules: &BTreeMap<String,Vec<[String; 8]>>, tries: usize) -> Vec<(f32, Vec<usize>)> {
    let mut lengths: Vec<usize> = students_to_valid_schedules.values().map(|schedules| schedules.len()).collect();
    let mut scores = Vec::new();
    let mut rng = thread_rng();
    for i in 0..tries {
        let random_perm = create_random_permutation(&lengths, &mut rng);
        let mut random_schedule_perm = Vec::new();
        for (i, (student, schedules)) in students_to_valid_schedules.iter().enumerate() {
            random_schedule_perm.push(&schedules[random_perm[i]]);
        }
        let score = calc_set_score(&random_schedule_perm);
        scores.push((score,random_perm));
        prune_scores(&mut scores);
    }
    return scores;
}

fn create_random_permutation(lengths: &Vec<usize>, rng: &mut ThreadRng) -> Vec<usize> {
    let mut out = Vec::new();
    for len in lengths {
        out.push(rng.gen_range(0,len))
    }
    return out;
}

fn calc_set_score(set: &Vec<&[String; 8]>) -> f32 {
    let mut score = 0f32;
    let students = set.len();
    for block in 0..8 {
        let mut unique_classes = HashSet::new();
        for schedule in set {
            unique_classes.insert(schedule[block].clone());
        }
        score += (students as f32/unique_classes.len() as f32).sqrt();
    }
    return score;
}

fn prune_scores(scores: &mut Vec<(f32, Vec<usize>)>) {
    if scores.len() > 10 {
        let mut min_index = 0;
        let mut min_score: f32 = 10000f32;
        for (i,score) in scores.iter().enumerate() {
            if score.0 < min_score {
                min_score = score.0;
                min_index = i;
            }
        }
        scores.remove(min_index);
    }
}

fn convert_index_scores(students_to_valid_schedules: &BTreeMap<String,Vec<[String; 8]>>, index_scores: Vec<(f32, Vec<usize>)>) -> Vec<(f32, Vec<(&String, &[String; 8])>)> {
    let mut out = Vec::new();
    for (score, permuation) in index_scores {
        let mut out_schedules = Vec::new();
        for (i, (student, schedules)) in students_to_valid_schedules.iter().enumerate() {
            out_schedules.push((student, &schedules[permuation[i]]));
        }
        out.push((score,out_schedules));
    }
    return out;
}