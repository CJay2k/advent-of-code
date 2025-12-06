use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("./assets/input.txt")?;
    let reader = BufReader::new(file);

    let mut ans: u32 = 0;

    let numbers_hm: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for line in reader.lines().flatten() {
        let mut v: Vec<u32> = vec![];

        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                v.push(c.to_digit(10).unwrap());
            } else {
                for num in numbers_hm.keys() {
                    if line[i..].starts_with(num) {
                        v.push(*numbers_hm.get(num).unwrap());
                    }
                }
            }
        }
        let joined_chars_as_number: u32 = format!("{}{}", v.first().unwrap(), v.last().unwrap())
            .parse()
            .unwrap();
        ans += joined_chars_as_number;
    }

    print!("{}", ans);

    Ok(())
}
