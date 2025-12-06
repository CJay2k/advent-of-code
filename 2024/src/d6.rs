#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn one_star() {
    let file_path: &str = "assets/d6_input.txt";
    let contents = fs::read_to_string(file_path).expect("");
    let mut lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

    let mut y = 0;
    let mut x = 0;
    let mut guard = '?';
    let mut result = 1;

    let moves: HashMap<char, (i32, i32, char)> = HashMap::from([
        ('^', (-1, 0, '>')),
        ('>', (0, 1, 'v')),
        ('v', (1, 0, '<')),
        ('<', (0, -1, '^')),
    ]);

    lines.iter().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, ch)| {
            if moves.contains_key(&ch) {
                y = i;
                x = j;
                guard = ch;
            }
        })
    });

    loop {
        let (y_move, x_move, guard_move) = moves.get(&guard).unwrap();
        let next_y = y as i32 + y_move;
        let next_x = x as i32 + x_move;

        if next_y < 0
            || next_y >= lines.len() as i32
            || next_x < 0
            || next_x >= lines[x].len() as i32
        {
            break;
        }

        if lines[next_y as usize].chars().nth(next_x as usize).unwrap() == '#' {
            guard = *guard_move;
        } else {
            y = next_y as usize;
            x = next_x as usize;

            if lines[y].chars().nth(x).unwrap() == '.' {
                lines[y].replace_range(x..=x, "X");
                result += 1;
            }
        }
    }

    println!("Result: {result}")
}

fn find_loop(lines: &[String], y: usize, x: usize, guard: char) -> (bool, HashSet<(usize, usize)>) {
    let mut _y = y;
    let mut _x: usize = x;
    let mut _guard = guard;
    let moves: HashMap<char, (i32, i32, char)> = HashMap::from([
        ('^', (-1, 0, '>')),
        ('>', (0, 1, 'v')),
        ('v', (1, 0, '<')),
        ('<', (0, -1, '^')),
    ]);
    let mut patch: HashSet<(usize, usize)> = HashSet::new();
    let mut seen: HashSet<(usize, usize, char)> = HashSet::new();

    loop {
        let (y_move, x_move, next_guard) = *moves.get(&_guard).unwrap();
        let next_y = _y as i32 + y_move;
        let next_x = _x as i32 + x_move;

        if next_y < 0
            || next_y >= lines.len() as i32
            || next_x < 0
            || next_x >= lines[_x].len() as i32
        {
            return (false, patch);
        }

        if lines[next_y as usize].chars().nth(next_x as usize).unwrap() == '#' {
            _guard = next_guard;
        } else {
            _y = next_y as usize;
            _x = next_x as usize;
        }
        patch.insert((_y, _x));
        if !seen.insert((_y, _x, _guard)) {
            return (true, patch);
        }
    }
}

pub fn two_stars() {
    let file_path: &str = "assets/d6_input.txt";
    let contents = fs::read_to_string(file_path).expect("");
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

    let mut y = 0;
    let mut x = 0;
    let guard = '^';
    let mut result = 0;

    lines.iter().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, ch)| {
            if ch == '^' {
                y = i;
                x = j;
            }
        })
    });

    let og_patch: HashSet<(usize, usize)> = find_loop(&lines.clone(), y, x, guard).1;

    og_patch.iter().for_each(|(i, j)| {
        let mut tmp = lines.clone();
        tmp[*i].replace_range(j..=j, "#");

        if find_loop(&tmp, y, x, guard).0 {
            result += 1;
        }
    });

    println!("Result: {result}")
}
