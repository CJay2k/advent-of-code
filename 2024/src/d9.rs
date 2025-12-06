#![allow(dead_code)]

use std::fs;

pub fn one_star() {
    let file_path: &str = "assets/d9_input.txt";
    let contents = fs::read_to_string(file_path).expect("");

    let mut disk = Vec::new();
    let mut space = false;
    for line in contents.lines() {
        let mut num = 0;
        for ch in line.chars() {
            for _ in 0..ch.to_digit(10).unwrap() {
                if space {
                    disk.push(-1);
                } else {
                    disk.push(num);
                }
            }

            if !space {
                num += 1;
            }
            space = !space;
        }
    }

    let disk_len: usize = disk.len();
    let mut i = 0;
    let mut j = disk_len - 1;
    loop {
        if disk[i] != -1 {
            i += 1;
            continue;
        }
        if disk[j] == -1 {
            j -= 1;
            continue;
        }

        disk.swap(i, j);
        i += 1;
        j -= 1;

        if i >= j {
            break;
        }
    }

    let mut result = 0;
    for (i, num) in disk.iter().enumerate() {
        if *num == -1 {
            break;
        }
        result += i as i64 * num;
    }

    println!("Result: {result}");
}

pub fn two_stars() {
    let file_path: &str = "assets/d9_input.txt";
    let contents = fs::read_to_string(file_path).expect("");

    let mut disk = Vec::new();
    let mut space = false;
    for line in contents.lines() {
        let mut num = 0;
        for ch in line.chars() {
            let x = ch.to_digit(10).unwrap();
            if x > 0 {
                if space {
                    disk.push((x, -1));
                } else {
                    disk.push((x, num));
                }
            }

            if !space {
                num += 1;
            }
            space = !space;
        }
    }

    let mut j = disk.len() - 1;
    loop {
        let mut i = 0;

        if j == 0 {
            break;
        }
        loop {
            if i >= j {
                break;
            }

            if disk[i].1 != -1 {
                i += 1;
                continue;
            }

            if disk[i].0 >= disk[j].0 && disk[j].1 != -1 {
                disk.swap(i, j);

                if disk[j].0 > disk[i].0 {
                    let remainder = disk[j].0 - disk[i].0;
                    disk[j].0 = disk[i].0;
                    disk.insert(i + 1, (remainder, -1));
                    j += 1;
                }

                break;
            }
            i += 1;
        }
        j -= 1;
    }

    let mut result = 0;
    let mut i = 0;
    for num in disk.iter() {
        for _ in 0..num.0 {
            if num.1 != -1 {
                result += i as i64 * num.1;
            }
            i += 1;
        }
    }
    println!("Result: {result}");
}
