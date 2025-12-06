#![allow(dead_code)]

use std::{collections::HashMap, fs};

pub fn one_star() {
    let file_path: &str = "assets/d5_input.txt";
    let contents = fs::read_to_string(file_path).expect("");
    let lines: Vec<&str> = contents.lines().collect();

    let mut result = 0;

    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    let mut empty_line = false;
    for line in lines {
        if line.is_empty() {
            empty_line = true;
            continue;
        }

        if !empty_line {
            let (v, k) = line.split_once("|").unwrap();
            map.entry(k.parse().unwrap())
                .or_default()
                .push(v.parse().unwrap());
        } else {
            updates.push(line.split(",").map(|x| x.parse().unwrap()).collect());
        }
    }

    for update in updates {
        let mut incorectly_ordered = false;
        update.iter().enumerate().for_each(|(i, x)| {
            for u in update[i + 1..].iter() {
                if !map.entry(*u).or_default().contains(x) {
                    incorectly_ordered = true;
                    break;
                }
            }
        });
        if !incorectly_ordered {
            result += update[update.len() / 2];
        }
    }

    println!("Result: {result}")
}

pub fn two_stars() {
    let file_path: &str = "assets/d5_input.txt";
    let contents = fs::read_to_string(file_path).expect("");
    let lines: Vec<&str> = contents.lines().collect();

    let mut result = 0;

    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    let mut empty_line = false;
    for line in lines {
        if line.is_empty() {
            empty_line = true;
            continue;
        }

        if !empty_line {
            let (v, k) = line.split_once("|").unwrap();
            map.entry(k.parse().unwrap())
                .or_default()
                .push(v.parse().unwrap());
        } else {
            updates.push(line.split(",").map(|x| x.parse().unwrap()).collect());
        }
    }

    for mut update in updates {
        let mut incorectly_ordered = false;

        for i in 0..update.len() {
            for j in i + 1..update.len() {
                if !map.entry(update[j]).or_default().contains(&update[i]) {
                    incorectly_ordered = true;
                    update.swap(i, j);
                }
            }
        }

        if incorectly_ordered {
            result += update[update.len() / 2];
        }
    }

    println!("Result: {result}")
}
