// If I'm not mistaken, I think it's just asking if the game is possible. Assume
// that each game is possible. If we see that there have been a certain numebr that
// is revealed to be less than possible. Change it to false.
// For each line, find the game ID, and get every number of colored balls and check if
// it is fine. How do I do this?
// Maybe, I can iterate through the v and match the color

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut answer = 0;

    for (index, line) in reader.lines().enumerate() {
        let string = line?;
        let v: Vec<&str> = string.split(&[':', ',', ';'][..]).collect();
        let mut possible = true;
        // println!("{:?}", v);

        for i in v[1..].iter() {
            // println!("{}", i);
            let string: Vec<&str> = i.trim().split(' ').collect();
            // println!("{:?}", string);

            let num_balls: u32 = string.get(0).unwrap().parse().unwrap();
            let color: &str = string.get(1).unwrap();

            match color {
                "red" => {
                    if num_balls > 12 {
                        possible = false;
                    }
                }
                "green" => {
                    if num_balls > 13 {
                        possible = false;
                    }
                }
                "blue" => {
                    if num_balls > 14 {
                        possible = false;
                    }
                }
                _ => println!("neither"),
            }
        }
        if possible {
            answer += index + 1;
        }
    }

    println!("Answer: {}", answer);
    Ok(())
}
