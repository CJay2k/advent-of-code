#![allow(dead_code)]

use std::{collections::HashMap, fs};

pub fn one_star() {
    let file_path: &str = "assets/d11_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut numbers = contents
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect::<Vec<u64>>();

    for _ in 0..25 {
        let mut temp_numbers = Vec::new();

        for num in numbers.iter() {
            if *num == 0 {
                temp_numbers.push(1);
            } else {
                let num_str = num.to_string();
                if num_str.len() % 2 == 0 {
                    let (first, second) = num_str.split_at(num_str.len() / 2);
                    temp_numbers.push(first.parse().unwrap());
                    temp_numbers.push(second.parse().unwrap());
                } else {
                    temp_numbers.push(num * 2024);
                }
            }
        }

        numbers = temp_numbers;
    }
    println!("Result: {}", numbers.len());
}

pub fn two_stars() {
    let file_path: &str = "assets/d11_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let numbers = contents
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect::<Vec<u64>>();

    let mut counter: HashMap<u64, u64> = numbers.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(*num).or_insert(0) += 1;
        acc
    });

    for _ in 0..75 {
        let mut temp_counter: HashMap<u64, u64> = HashMap::new();
        for (num, count) in counter.clone() {
            if num == 0 {
                temp_counter.insert(1, temp_counter.get(&1).unwrap_or(&0) + count);
            } else {
                let num_str = num.to_string();
                if num_str.len() % 2 == 0 {
                    let (first, second) = num_str.split_at(num_str.len() / 2);
                    let first_num = first.parse().unwrap();
                    let second_num = second.parse().unwrap();
                    temp_counter.insert(
                        first_num,
                        temp_counter.get(&first_num).unwrap_or(&0) + count,
                    );
                    temp_counter.insert(
                        second_num,
                        temp_counter.get(&second_num).unwrap_or(&0) + count,
                    );
                } else {
                    temp_counter.insert(
                        num * 2024,
                        temp_counter.get(&(num * 2024)).unwrap_or(&0) + count,
                    );
                }
            }
        }

        counter = temp_counter;
    }

    println!("Result: {}", counter.values().sum::<u64>());
}
