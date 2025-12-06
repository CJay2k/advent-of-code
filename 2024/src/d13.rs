#![allow(dead_code)]

use std::fs;

pub fn one_star() {
    let file_path: &str = "assets/d13_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let mut result: u32 = 0;
    let mut machines: Vec<Vec<(u32, u32)>> = Vec::new();

    let mut machine: Vec<(u32, u32)> = Vec::new();
    contents.lines().for_each(|line| {
        if !line.is_empty() {
            let x: u32 = line
                .get(line.find("X").unwrap() + 2..line.find(",").unwrap())
                .unwrap()
                .parse()
                .unwrap();

            let y = line
                .get(line.find("Y").unwrap() + 2..line.len())
                .unwrap()
                .parse()
                .unwrap();
            machine.push((x, y));

            if machine.len() == 3 {
                machines.push(machine.clone());
                machine = Vec::new();
            }
        }
    });

    for machine in machines {
        let mut wins: Vec<u32> = Vec::new();
        for a in 0..100 {
            for b in 0..100 {
                if machine[0].0 * a + machine[1].0 * b == machine[2].0
                    && machine[0].1 * a + machine[1].1 * b == machine[2].1
                {
                    wins.push(a * 3 + b);
                }
            }
        }
        result += *wins.iter().min().unwrap_or(&0);
    }

    println!("Result: {}", result);
}

pub fn two_stars() {
    let file_path: &str = "assets/d13_input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let mut result: i64 = 0;
    let mut machines: Vec<Vec<(i64, i64)>> = Vec::new();

    let mut machine: Vec<(i64, i64)> = Vec::new();
    contents.lines().for_each(|line| {
        if !line.is_empty() {
            let mut x: i64 = line
                .get(line.find("X").unwrap() + 2..line.find(",").unwrap())
                .unwrap()
                .parse()
                .unwrap();

            let mut y: i64 = line
                .get(line.find("Y").unwrap() + 2..line.len())
                .unwrap()
                .parse()
                .unwrap();

            if line.contains("Prize") {
                x += 10000000000000;
                y += 10000000000000;
            }
            machine.push((x, y));

            if machine.len() == 3 {
                machines.push(machine.clone());
                machine = Vec::new();
            }
        }
    });

    for machine in machines {
        let ax = machine[0].0;
        let ay = machine[0].1;

        let bx = machine[1].0;
        let by = machine[1].1;

        let px = machine[2].0;
        let py = machine[2].1;

        if (py * ax - px * ay) % (by * ax - bx * ay) == 0 {
            let b = (py * ax - px * ay) / (by * ax - bx * ay);

            if (px - b * bx) % ax == 0 {
                let a = (px - b * bx) / ax;
                result += a * 3 + b;
            }
        }
    }

    println!("Result: {}", result);
}
