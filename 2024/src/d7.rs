#![allow(dead_code)]

use std::{collections::HashSet, fs};

pub fn one_star() {
    let file_path: &str = "assets/d7_input.txt";
    let contents = fs::read_to_string(file_path).expect("");
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    let mut result = 0;

    let parsed: Vec<(i64, Vec<i64>)> = lines
        .iter()
        .map(|line| {
            let x: Vec<_> = line.split(":").collect();

            (
                x[0].parse().unwrap(),
                x[1].split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            )
        })
        .collect();

    for row in parsed.iter() {
        let numbers = &row.1;

        let mut sub_results1: Vec<i64> = vec![numbers[0]];
        let mut sub_results2: Vec<i64> = Vec::new();
        numbers.iter().skip(1).for_each(|num| {
            while let Some(first) = sub_results1.pop() {
                sub_results2.push(first + num);
                sub_results2.push(first * num);
            }

            sub_results1 = sub_results2.clone();
        });

        if sub_results1.contains(&row.0) {
            result += row.0;
        }
    }

    println!("Result: {result}");
}

pub fn two_stars() {
    let file_path: &str = "assets/d7_input.txt";
    let contents = fs::read_to_string(file_path).expect("");
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    let mut result = 0;

    let parsed: Vec<(i64, Vec<i64>)> = lines
        .iter()
        .map(|line| {
            let x: Vec<_> = line.split(":").collect();

            (
                x[0].parse().unwrap(),
                x[1].split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            )
        })
        .collect();

    for row in parsed.iter() {
        let numbers = &row.1;

        let mut sub_results1: HashSet<i64> = HashSet::from([numbers[0]]);
        let mut sub_results2: HashSet<i64> = HashSet::new();
        numbers.iter().skip(1).for_each(|num| {
            for first in sub_results1.iter() {
                sub_results2.insert(first + num);
                sub_results2.insert(first * num);
                sub_results2.insert(format!("{first}{num}").parse().unwrap());
            }

            sub_results1 = sub_results2.clone();
            sub_results2.clear();
        });

        if sub_results1.contains(&row.0) {
            result += row.0;
        }
    }
    println!("Result: {result}");
}
