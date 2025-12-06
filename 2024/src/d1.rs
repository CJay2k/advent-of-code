#![allow(dead_code)]
use std::{collections::HashMap, fs};

pub fn one_star() {
    let file_path: &str = "assets/d1_input.txt";

    let contents = fs::read_to_string(file_path).expect("");

    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();
    for line in contents.lines() {
        let parsed_numbers: Vec<u32> = line
            .split_whitespace()
            .map(|num: &str| num.parse().unwrap())
            .collect();

        left_list.push(parsed_numbers[0]);
        right_list.push(parsed_numbers[1]);
    }

    left_list.sort();
    right_list.sort();

    let mut result: u32 = 0;
    for (a, b) in left_list.iter().zip(right_list) {
        result += a.abs_diff(b);
    }

    println!("Result: {result}")
}

pub fn two_stars() {
    let file_path: &str = "assets/d1_input.txt";

    let contents = fs::read_to_string(file_path).expect("");

    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();
    for line in contents.lines() {
        let parsed_numbers: Vec<u32> = line
            .split_whitespace()
            .map(|num: &str| num.parse().unwrap())
            .collect();

        left_list.push(parsed_numbers[0]);
        right_list.push(parsed_numbers[1]);
    }

    let mut right_list_frequency: HashMap<u32, u32> = HashMap::new();
    for num in right_list {
        right_list_frequency
            .entry(num)
            .and_modify(|counter: &mut u32| *counter += 1)
            .or_insert(1);
    }

    let mut result: u32 = 0;
    for num in left_list.iter() {
        result += num * right_list_frequency.get(num).unwrap_or(&0);
    }

    println!("Result: {result}")
}
