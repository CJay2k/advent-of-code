#![allow(dead_code)]

use std::fs;

pub fn one_star() {
    let file_path: &str = "assets/d14_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut robots: Vec<((i32, i32), (i32, i32))> = Vec::new();

    for line in contents.lines() {
        let mut split = line.split_whitespace();

        let p = split.next().unwrap();
        let v = split.next().unwrap();

        let mut p = p.trim_start_matches("p=").split(",");
        let mut v = v.trim_start_matches("v=").split(",");

        let p_x = p.next().unwrap().parse::<i32>().unwrap();
        let p_y = p.next().unwrap().parse::<i32>().unwrap();

        let v_x = v.next().unwrap().parse::<i32>().unwrap();
        let v_y = v.next().unwrap().parse::<i32>().unwrap();

        robots.push(((p_x, p_y), (v_x, v_y)));
    }

    const WIDTH: usize = 101;
    const HEIGHT: usize = 103;

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.0 .0 += robot.1 .0;
            robot.0 .1 += robot.1 .1;

            robot.0 .0 = robot.0 .0.rem_euclid(WIDTH as i32);
            robot.0 .1 = robot.0 .1.rem_euclid(HEIGHT as i32);
        }
    }

    robots.iter().for_each(|robot| println!("{robot:?}"));

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for robot in robots {
        let in_top = (0..HEIGHT / 2).contains(&(robot.0 .1 as usize));
        let in_bottom = (HEIGHT / 2 + 1..HEIGHT).contains(&(robot.0 .1 as usize));
        let in_left = (0..WIDTH / 2).contains(&(robot.0 .0 as usize));
        let in_right = (WIDTH / 2 + 1..WIDTH).contains(&(robot.0 .0 as usize));

        if in_left && in_top {
            q1 += 1;
            println!("Q1 {} {}", robot.0 .0, robot.0 .1);
        } else if in_right && in_top {
            q2 += 1;
            println!("Q2 {} {}", robot.0 .0, robot.0 .1);
        } else if in_left && in_bottom {
            q3 += 1;
            println!("Q3 {} {}", robot.0 .0, robot.0 .1);
        } else if in_right && in_bottom {
            q4 += 1;
            println!("Q4 {} {}", robot.0 .0, robot.0 .1);
        }
    }

    println!("{q1} {q2} {q3} {q4}");

    let result = q1 * q2 * q3 * q4;
    println!("Result: {}", result);
}

pub fn two_stars() {
    let file_path: &str = "assets/d14_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let mut robots: Vec<((i32, i32), (i32, i32))> = Vec::new();

    for line in contents.lines() {
        let mut split = line.split_whitespace();

        let p = split.next().unwrap();
        let v = split.next().unwrap();

        let mut p = p.trim_start_matches("p=").split(",");
        let mut v = v.trim_start_matches("v=").split(",");

        let p_x = p.next().unwrap().parse::<i32>().unwrap();
        let p_y = p.next().unwrap().parse::<i32>().unwrap();

        let v_x = v.next().unwrap().parse::<i32>().unwrap();
        let v_y = v.next().unwrap().parse::<i32>().unwrap();

        robots.push(((p_x, p_y), (v_x, v_y)));
    }

    const WIDTH: usize = 101;
    const HEIGHT: usize = 103;

    for i in 1..1000000000 {
        let mut board = vec![vec!["."; HEIGHT]; WIDTH];

        for robot in robots.iter_mut() {
            robot.0 .0 += robot.1 .0;
            robot.0 .1 += robot.1 .1;

            robot.0 .0 = robot.0 .0.rem_euclid(WIDTH as i32);
            robot.0 .1 = robot.0 .1.rem_euclid(HEIGHT as i32);

            board[robot.0 .0 as usize][robot.0 .1 as usize] = "#";
        }

        let mut end_cords = robots
            .iter()
            .map(|robot| (robot.0 .0, robot.0 .1))
            .collect::<Vec<_>>();

        end_cords.sort_by_key(|robot| (robot.0, robot.1));

        let mut consecutive = 0;
        for pair in end_cords.windows(2) {
            if pair[0].0 == pair[1].0 && pair[0].1 == pair[1].1 || pair[0].1 + 1 == pair[1].1 {
                consecutive += 1;
            } else {
                consecutive = 0;
            }
            if consecutive == 30 {
                break;
            }
        }

        if consecutive == 30 {
            println!("Step {}", i);
            board.iter().for_each(|row| println!("{:?}", row.concat()));
            break;
        }
    }
}
