#![allow(dead_code)]

use std::fs;

fn always_descending(numbers: &[u32]) -> bool {
    for i in 1..numbers.len() {
        if numbers[i - 1] >= numbers[i] || ![1, 2, 3].contains(&numbers[i - 1].abs_diff(numbers[i]))
        {
            return false;
        }
    }
    true
}

fn always_ascending(numbers: &[u32]) -> bool {
    for i in 1..numbers.len() {
        if numbers[i - 1] <= numbers[i] || ![1, 2, 3].contains(&numbers[i - 1].abs_diff(numbers[i]))
        {
            return false;
        }
    }
    true
}

pub fn one_star() {
    let file_path: &str = "assets/d2_input.txt";

    let contents = fs::read_to_string(file_path).expect("");

    let mut result: u32 = 0;
    for line in contents.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|num: &str| num.parse().unwrap())
            .collect();

        if always_descending(&numbers) || always_ascending(&numbers) {
            result += 1;
        }
    }

    println!("Result: {result}")
}

pub fn two_stars() {
    let file_path: &str = "assets/d2_input.txt";

    let contents = fs::read_to_string(file_path).expect("");

    let mut result: u32 = 0;
    for line in contents.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|num: &str| num.parse().unwrap())
            .collect();

        for i in 0..numbers.len() {
            if always_descending(&[&numbers[..i], &numbers[i + 1..]].concat())
                || always_ascending(&[&numbers[..i], &numbers[i + 1..]].concat())
            {
                result += 1;
                break;
            }
        }
    }

    println!("Result: {result}")
}
