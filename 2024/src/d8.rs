#![allow(dead_code)]

use std::{collections::HashMap, fs};

pub fn one_star() {
    let file_path: &str = "assets/d8_input.txt";
    let contents = fs::read_to_string(file_path).expect("");
    let mut lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    let mut anthenas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| {
            if ch != '.' {
                anthenas.entry(ch).or_default().push((y, x));
            }
        });
    });

    anthenas.iter().for_each(|(_, v)| {
        for i in 0..v.len() {
            for j in i + 1..v.len() {
                let (y1, x1) = v[i];
                let (y2, x2) = v[j];

                let diff = (y2 as i32 - y1 as i32, x2 as i32 - x1 as i32);

                let y4 = y1 as i32 - diff.0;
                let x4 = x1 as i32 - diff.1;

                let y3 = y2 as i32 + diff.0;
                let x3 = x2 as i32 + diff.1;

                println!("{y3} {x3} {y4} {x4}");

                if y3 >= 0 && y3 < lines.len() as i32 && x3 >= 0 && x3 < lines.len() as i32 {
                    lines[y3 as usize].replace_range(x3 as usize..=x3 as usize, "#");
                }

                if y4 >= 0 && y4 < lines.len() as i32 && x4 >= 0 && x4 < lines.len() as i32 {
                    lines[y4 as usize].replace_range(x4 as usize..=x4 as usize, "#");
                }
            }
        }
    });

    lines.iter().for_each(|line| println!("{line}"));

    let result: usize = lines
        .iter()
        .map(|line| line.chars().filter(|&ch| ch == '#').count())
        .sum();

    println!("Result: {result}");
}

pub fn two_stars() {
    let file_path: &str = "assets/d8_input.txt";
    let contents = fs::read_to_string(file_path).expect("");
    let mut lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    let mut anthenas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| {
            if ch != '.' {
                anthenas.entry(ch).or_default().push((y, x));
            }
        });
    });

    anthenas.iter().for_each(|(_, v)| {
        for i in 0..v.len() {
            for j in i + 1..v.len() {
                let (y1, x1) = v[i];
                let (y2, x2) = v[j];

                let diff = (y2 as i32 - y1 as i32, x2 as i32 - x1 as i32);

                let mut y3: i32 = y2 as i32;
                let mut x3: i32 = x2 as i32;
                loop {
                    y3 += diff.0;
                    x3 += diff.1;

                    if y3 < 0 || y3 >= lines.len() as i32 || x3 < 0 || x3 >= lines.len() as i32 {
                        break;
                    }
                    lines[y3 as usize].replace_range(x3 as usize..=x3 as usize, "#");
                }

                let mut y4: i32 = y1 as i32;
                let mut x4: i32 = x1 as i32;
                loop {
                    y4 -= diff.0;
                    x4 -= diff.1;

                    if y4 < 0 || y4 >= lines.len() as i32 || x4 < 0 || x4 >= lines.len() as i32 {
                        break;
                    }
                    lines[y4 as usize].replace_range(x4 as usize..=x4 as usize, "#");
                }
            }
        }
    });

    lines.iter().for_each(|line| println!("{line}"));

    let result: usize = lines
        .iter()
        .map(|line| line.chars().filter(|&ch| ch != '.').count())
        .sum();

    println!("Result: {result}");
}
