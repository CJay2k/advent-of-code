use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("./assets/input.txt")?;
    let reader = BufReader::new(file);

    let mut ans: u32 = 0;

    for line in reader.lines().flatten() {
        let mut v: Vec<u32> = vec![];

        for c in line.chars() {
            if c.is_numeric() {
                v.push(c.to_digit(10).unwrap());
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
