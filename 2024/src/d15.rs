#![allow(dead_code)]

use std::{collections::HashMap, fs};

fn find_free_space(map: &[Vec<char>], _move: (i32, i32), robot_position: (i32, i32)) -> (i32, i32) {
    let mut pos = robot_position;

    loop {
        pos = (pos.0 + _move.0, pos.1 + _move.1);

        if pos.0 >= 0 && pos.1 >= 0 && pos.0 < map.len() as i32 && pos.1 < map[0].len() as i32 {
            let next_ch = map[pos.0 as usize][pos.1 as usize];

            if next_ch == '.' {
                return pos;
            } else if next_ch == '#' {
                return (-1, -1);
            }
        }
    }
}

pub fn one_star() {
    let file_path: &str = "assets/d15_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut map: Vec<Vec<char>> = vec![];
    let moves: HashMap<char, (i32, i32)> =
        HashMap::from([('^', (-1, 0)), ('v', (1, 0)), ('<', (0, -1)), ('>', (0, 1))]);

    let mut robot_position: (i32, i32) = (0, 0);
    let mut part2 = false;
    for (y, line) in contents.lines().enumerate() {
        if line.is_empty() {
            part2 = true;
        }

        if !part2 {
            map.push(line.chars().collect());
            if line.contains("@") {
                robot_position = (y as i32, line.find("@").unwrap() as i32);
                map[y][line.find("@").unwrap()] = '.';
            }
        } else {
            for m in line.chars() {
                let _move = moves.get(&m).unwrap();
                let next_y = robot_position.0 + _move.0;
                let next_x = robot_position.1 + _move.1;
                if next_y >= 0
                    && next_x >= 0
                    && next_y < map.len() as i32
                    && next_x < map[0].len() as i32
                {
                    let next_ch = map[next_y as usize][next_x as usize];
                    if next_ch == '.' {
                        robot_position = (next_y, next_x);
                    } else if next_ch == 'O' {
                        let free_space = find_free_space(&map, *_move, robot_position);
                        if free_space.0 != -1 {
                            map[next_y as usize][next_x as usize] = '.';
                            map[free_space.0 as usize][free_space.1 as usize] = 'O';
                            robot_position = (next_y, next_x);
                        }
                    }
                }
            }
        }
    }

    let mut result = 0;

    for (y, line) in map.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch == 'O' {
                result += 100 * y + x;
            }
        }
    }

    println!("Result: {}", result);
}

fn find_free_space2(
    map: &[Vec<char>],
    _move: (i32, i32),
    robot_position: (i32, i32),
) -> (i32, i32) {
    let mut pos = robot_position;

    let mut i = 0;
    loop {
        pos = (pos.0 + _move.0, pos.1 + _move.1);

        if pos.0 >= 0 && pos.1 >= 0 && pos.0 < map.len() as i32 && pos.1 < map[0].len() as i32 {
            let next_ch = map[pos.0 as usize][pos.1 as usize];

            if next_ch == '.' {
                return pos;
            } else if next_ch == '#' || i >= 4 {
                return (-1, -1);
            }
        }

        i += 1;
    }
}

pub fn two_stars() {
    // let file_path: &str = "assets/d15_input.txt";
    let file_path: &str = "assets/test.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut map: Vec<Vec<char>> = vec![];
    let moves: HashMap<char, (i32, i32)> =
        HashMap::from([('^', (-1, 0)), ('v', (1, 0)), ('<', (0, -1)), ('>', (0, 1))]);

    let mut robot_position: (i32, i32) = (0, 0);
    let mut part2 = false;
    for (y, line) in contents.lines().enumerate() {
        if line.is_empty() {
            part2 = true;
        }

        if !part2 {
            let mut l = vec![];
            for (i, c) in line.chars().enumerate() {
                if c == '@' {
                    l.push('.');
                    l.push('.');
                    robot_position = (y as i32, 2 * i as i32);
                } else if c == 'O' {
                    l.push('[');
                    l.push(']');
                } else {
                    l.push(c);
                    l.push(c);
                }
            }
            map.push(l);
        } else {
            map.iter()
                .for_each(|line| println!("{}", line.iter().collect::<String>()));
        }
    }

    // let mut result = 0;

    // for (y, line) in map.iter().enumerate() {
    //     for (x, ch) in line.iter().enumerate() {
    //         if *ch == 'O' {
    //             result += 100 * y + x;
    //         }
    //     }
    // }

    map.iter()
        .for_each(|line| println!("{}", line.iter().collect::<String>()));

    println!("{:?}", moves);
    println!("{:?}", robot_position);

    // println!("Result: {}", result);
}
