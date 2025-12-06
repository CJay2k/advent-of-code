use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./assets/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut ans: u32 = 0;

    for line in reader.lines().flatten() {
        let mut min_dices = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        let (_, all_rolls_str) = line.split_once(":").unwrap();

        let all_rolls: Vec<_> = all_rolls_str.split(";").collect();

        for rolls_str in all_rolls {
            let rolls: Vec<_> = rolls_str.trim().split(",").collect();
            for roll in rolls {
                let (count_str, color) = roll.trim().split_once(" ").unwrap();

                let count = count_str.parse::<u32>().unwrap();
                if count > *min_dices.get(color).unwrap(){
                    min_dices.insert(color, count) ;
                }
            }
        }
        ans += min_dices.values().product::<u32>();
    }

    println!("{}", ans)
}
