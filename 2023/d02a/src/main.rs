use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let file = File::open("./assets/input.txt")?;
    let reader = BufReader::new(file);

    let mut ans = 0;

    let max_dices = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for line in reader.lines().flatten() {
        let v: Vec<&str> = line.split(":").collect();
        let game_id: u32 = v[0].split(" ").collect::<Vec<_>>()[1].parse().unwrap();
        let rounds: Vec<_> = v[1].split(";").collect();
        
        let mut valid: bool = true;
        for round in rounds {
            let all_dices_revealed: Vec<_> = round.split(",").collect();

            for dices in all_dices_revealed {
                let (count, color) = dices.trim().split_once(" ").unwrap();

                if count.parse::<u32>().unwrap() > *max_dices.get(color).unwrap() {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            ans += game_id;
        }
    }

    println!("{}", ans);

    Ok(())
}
