#![allow(dead_code)]

use std::{cmp::Ordering, fs};

pub fn one_star() {
    let file_path: &str = "assets/d3_input.txt";

    let contents = fs::read_to_string(file_path).expect("");

    let mut result: i32 = 0;
    for item in contents.split("mul(") {
        let mut first_num = String::from("");
        let mut second_num = String::from("");
        let mut coma_found = false;
        for c in item.chars() {
            if c.is_numeric() {
                if !coma_found {
                    first_num.push(c);
                } else {
                    second_num.push(c);
                }
            } else if c == ',' && !first_num.is_empty() && second_num.is_empty() {
                coma_found = true;
            } else if c == ')' && !first_num.is_empty() && coma_found && !first_num.is_empty() {
                result += first_num.parse::<i32>().unwrap() * second_num.parse::<i32>().unwrap();
                break;
            } else {
                break;
            }
        }
    }

    println!("Result: {result}")
}

pub fn two_stars() {
    let file_path: &str = "assets/d3_input.txt";

    let contents = fs::read_to_string(file_path).expect("");

    let mut result: i32 = 0;
    let mut mul_enabled = true;
    for item in contents.split("mul(") {
        let mut first_num = String::from("");
        let mut second_num = String::from("");
        let mut coma_found = false;

        if mul_enabled {
            for c in item.chars() {
                if c.is_numeric() {
                    if !coma_found {
                        first_num.push(c);
                    } else {
                        second_num.push(c);
                    }
                } else if c == ',' && !first_num.is_empty() && second_num.is_empty() {
                    coma_found = true;
                } else if c == ')' && !first_num.is_empty() && coma_found && !first_num.is_empty() {
                    result +=
                        first_num.parse::<i32>().unwrap() * second_num.parse::<i32>().unwrap();
                    break;
                } else {
                    break;
                }
            }
        }

        let dont_index = item.rfind("don't()").unwrap_or(0);
        let do_index = item.rfind("do()").unwrap_or(0);
        mul_enabled = match do_index.cmp(&dont_index) {
            Ordering::Greater => true,
            Ordering::Less => false,
            Ordering::Equal => mul_enabled,
        }
    }

    println!("Result: {result}")
}
