#![allow(dead_code)]

use std::{collections::HashSet, fs};

fn rec(map: &[Vec<u32>], y: usize, x: usize, visited: &mut HashSet<(usize, usize)>) {
    if map[y][x] == 9 {
        visited.insert((y, x));
        return;
    }

    let moves = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (dy, dx) in moves {
        let next_y = y as i32 + dy;
        let next_x = x as i32 + dx;

        if next_y >= 0
            && next_x >= 0
            && next_y < map.len() as i32
            && next_x < map[0].len() as i32
            && map[next_y as usize][next_x as usize] == map[y][x] + 1
        {
            rec(map, next_y as usize, next_x as usize, visited);
        }
    }
}

pub fn one_star() {
    let file_path: &str = "assets/d10_input.txt";
    let contents = fs::read_to_string(file_path).expect("");
    let mut result = 0;

    let mut map: Vec<Vec<u32>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<u32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        map.push(row);
    }

    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, num)| {
            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            if *num == 0 {
                rec(&map, y, x, &mut visited);
            }
            result += visited.len();
        });
    });

    println!("Result: {result}");
}

fn rec2(map: &[Vec<u32>], y: usize, x: usize) -> i32 {
    if map[y][x] == 9 {
        return 1;
    }

    let moves = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut result = 0;
    for (dy, dx) in moves {
        let next_y = y as i32 + dy;
        let next_x = x as i32 + dx;

        if next_y >= 0
            && next_x >= 0
            && next_y < map.len() as i32
            && next_x < map[0].len() as i32
            && map[next_y as usize][next_x as usize] == map[y][x] + 1
        {
            result += rec2(map, next_y as usize, next_x as usize)
        }
    }

    result
}

pub fn two_stars() {
    let file_path: &str = "assets/d10_input.txt";
    let contents = fs::read_to_string(file_path).expect("");
    let mut result = 0;

    let mut map: Vec<Vec<u32>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<u32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        map.push(row);
    }

    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, num)| {
            if *num == 0 {
                result += rec2(&map, y, x);
            }
        });
    });

    println!("Result: {result}");
}
