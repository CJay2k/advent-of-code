#![allow(dead_code)]

use std::{char, collections::HashSet, fs};

fn rec(map: &[Vec<char>], y: usize, x: usize, seen: &mut HashSet<(usize, usize)>) -> (u32, u32) {
    let moves = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut area = 1;
    let mut perim = 0;
    seen.insert((y, x));

    for m in moves {
        let next_y = y as i32 + m.0;
        let next_x = x as i32 + m.1;

        let in_bounds =
            next_y >= 0 && next_x >= 0 && next_y < map.len() as i32 && next_x < map[0].len() as i32;

        if in_bounds && map[y][x] == map[next_y as usize][next_x as usize] {
            if !seen.contains(&(next_y as usize, next_x as usize)) {
                let (area2, perim2) = rec(map, next_y as usize, next_x as usize, seen);
                area += area2;
                perim += perim2;
            }
        } else {
            perim += 1;
        }
    }

    (area, perim)
}

pub fn one_star() {
    let file_path: &str = "assets/d12_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut result = 0;
    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, _)| {
            if !seen.contains(&(y, x)) {
                let (area, perim) = rec(&map, y, x, &mut seen);
                result += area * perim;
            }
        });
    });

    println!("Result: {}", result);
}

fn rec2(
    map: &[Vec<char>],
    y: usize,
    x: usize,
    seen: &mut HashSet<(usize, usize)>,
) -> (u32, Vec<(u32, usize, usize)>) {
    let moves = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut area = 1;
    let mut sides: Vec<(u32, usize, usize)> = Vec::new();
    seen.insert((y, x));

    for (i, m) in moves.iter().enumerate() {
        let next_y = y as i32 + m.0;
        let next_x = x as i32 + m.1;

        let in_bounds =
            next_y >= 0 && next_x >= 0 && next_y < map.len() as i32 && next_x < map[0].len() as i32;

        if in_bounds && map[y][x] == map[next_y as usize][next_x as usize] {
            if !seen.contains(&(next_y as usize, next_x as usize)) {
                let (area2, sides2) = rec2(map, next_y as usize, next_x as usize, seen);
                area += area2;
                sides.extend(sides2);
            }
        } else {
            sides.push((i as u32, y, x));
        }
    }

    (area, sides)
}

pub fn two_stars() {
    let file_path: &str = "assets/d12_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut result = 0;
    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, _)| {
            if !seen.contains(&(y, x)) {
                let (area, sides) = rec2(&map, y, x, &mut seen);
                let mut vec1: Vec<(usize, usize)> = Vec::new();
                let mut vec2: Vec<(usize, usize)> = Vec::new();
                let mut vec3: Vec<(usize, usize)> = Vec::new();
                let mut vec4: Vec<(usize, usize)> = Vec::new();

                let mut sides_count = 4;

                for side in sides.iter() {
                    if side.0 == 0 {
                        vec1.push((side.1, side.2));
                    } else if side.0 == 1 {
                        vec2.push((side.1, side.2));
                    } else if side.0 == 2 {
                        vec3.push((side.1, side.2));
                    } else if side.0 == 3 {
                        vec4.push((side.1, side.2));
                    }
                }
                vec1.sort_by(
                    |(b1, a1), (b2, a2)| {
                        if a1 == a2 {
                            b1.cmp(b2)
                        } else {
                            a1.cmp(a2)
                        }
                    },
                );
                vec2.sort_by(
                    |(b1, a1), (b2, a2)| {
                        if a1 == a2 {
                            b1.cmp(b2)
                        } else {
                            a1.cmp(a2)
                        }
                    },
                );
                vec3.sort_by(
                    |(a1, b1), (a2, b2)| {
                        if a1 == a2 {
                            b1.cmp(b2)
                        } else {
                            a1.cmp(a2)
                        }
                    },
                );
                vec4.sort_by(
                    |(a1, b1), (a2, b2)| {
                        if a1 == a2 {
                            b1.cmp(b2)
                        } else {
                            a1.cmp(a2)
                        }
                    },
                );

                for i in 0..vec1.len() - 1 {
                    if vec1[i].1 != vec1[i + 1].1 || vec1[i].0 + 1 != vec1[i + 1].0 {
                        sides_count += 1;
                    }
                }
                for i in 0..vec2.len() - 1 {
                    if vec2[i].1 != vec2[i + 1].1 || vec2[i].0 + 1 != vec2[i + 1].0 {
                        sides_count += 1;
                    }
                }

                for i in 0..vec3.len() - 1 {
                    if vec3[i].0 != vec3[i + 1].0 || vec3[i].1 + 1 != vec3[i + 1].1 {
                        sides_count += 1;
                    }
                }

                for i in 0..vec4.len() - 1 {
                    if vec4[i].0 != vec4[i + 1].0 || vec4[i].1 + 1 != vec4[i + 1].1 {
                        sides_count += 1;
                    }
                }

                result += sides_count * area;
            }
        });
    });
    println!("Result: {}", result);
}
