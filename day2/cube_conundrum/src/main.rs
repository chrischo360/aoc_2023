// If I'm not mistaken, I think it's just asking if the game is possible. Assume
// that each game is possible. If we see that there have been a certain numebr that
// is revealed to be less than possible. Change it to false.
// For each line, find the game ID, and get every number of colored balls and check if
// it is fine. How do I do this?
// Maybe, I can iterate through the v and match the color
// Part 2: Just find the max?

use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut answer = 0;

    for (index, line) in reader.lines().enumerate() {
        let string = line?;
        let v: Vec<&str> = string.split(&[':', ',', ';'][..]).collect();
        let mut max_blue = 1;
        let mut max_red = 1;
        let mut max_green = 1;

        for i in v[1..].iter() {
            // println!("{}", i);
            let string: Vec<&str> = i.trim().split(' ').collect();
            // println!("{:?}", string);

            let num_balls: u32 = string.get(0).unwrap().parse().unwrap();
            let color: &str = string.get(1).unwrap();

            match color {
                "red" => {
                    max_red = cmp::max(max_red, num_balls);
                }
                "green" => {
                    max_green = cmp::max(max_green, num_balls);
                }
                "blue" => {
                    max_blue = cmp::max(max_blue, num_balls);
                }
                _ => println!("neither"),
            }
        }
        answer += (max_green * max_blue * max_red);
    }

    println!("Answer: {}", answer);
    Ok(())
}
