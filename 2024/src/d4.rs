#![allow(dead_code)]

use std::fs;

pub fn one_star() {
    let file_path: &str = "assets/d4_input.txt";
    let contents = fs::read_to_string(file_path).expect("");
    let lines: Vec<&str> = contents.split_whitespace().collect();

    let mut words: Vec<String> = Vec::new();
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if j + 3 < lines.len() {
                words.push(lines[i][j..=j + 3].to_string());
            }
            if i + 3 < lines.len() {
                let mut word = String::new();
                for k in 0..=3 {
                    word.push(lines[i + k].chars().nth(j).unwrap());
                }
                words.push(word);
            }
            if i + 3 < lines.len() && j + 3 < lines.len() {
                let mut word = String::new();
                for k in 0..=3 {
                    word.push(lines[i + k].chars().nth(j + k).unwrap());
                }
                words.push(word);
            }

            if i + 3 < lines.len() && j >= 3 {
                let mut word = String::new();
                for k in 0..=3 {
                    word.push(lines[i + k].chars().nth(j - k).unwrap());
                }
                words.push(word);
            }
        }
    }

    let result = words
        .iter()
        .filter(|word| *word == "XMAS" || *word == "SAMX")
        .count() as u32;

    println!("Result: {result}")
}

pub fn two_stars() {
    let file_path: &str = "assets/d4_input.txt";

    let contents = fs::read_to_string(file_path).expect("");
    let lines: Vec<&str> = contents.split_whitespace().collect();
    let mut result = 0;
    for i in 1..lines.len() {
        for j in 1..lines.len() {
            if i + 1 < lines.len() && j + 1 < lines.len() {
                let mut word1 = String::new();
                let mut word2 = String::new();
                let moves1: [(i32, i32); 3] = [(1, 1), (0, 0), (-1, -1)];
                let moves2: [(i32, i32); 3] = [(1, -1), (0, 0), (-1, 1)];
                for (a, b) in moves1.iter() {
                    word1.push(
                        lines[(i as i32 + a) as usize]
                            .chars()
                            .nth((j as i32 + b) as usize)
                            .unwrap(),
                    );
                }

                if word1 == "MAS" || word1 == "SAM" {
                    for (a, b) in moves2.iter() {
                        word2.push(
                            lines[(i as i32 + a) as usize]
                                .chars()
                                .nth((j as i32 + b) as usize)
                                .unwrap(),
                        );
                    }

                    if word2 == "MAS" || word2 == "SAM" {
                        result += 1;
                    }
                }
            }
        }
    }

    println!("Result: {result}")
}
