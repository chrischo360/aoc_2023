// 1) Take in line by line
// 2) Loop through the line
// 3) Find the first and last digit

use std::fs::File;
use std::io::{self, BufRead};
use std::option::Option;
use std::path::Path;

fn main() {
    const RADIX: u32 = 10;
    let mut answer = 0;
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let mut first: Option<char> = None;
            let mut second: Option<char> = None;
            for c in line.chars() {
                if c.is_numeric() {
                    //let digit = c.to_digit(RADIX).unwrap_or(0) as u32;
                    if first == None {
                        first = Some(c);
                    }
                    second = Some(c);
                }
            }
            if let Some(first_val) = first {
                // println!("First: {}", first_val);
                if let Some(second_val) = second {
                    let res = format!("{}{}", first_val, second_val);
                    let digit = res.parse::<u32>().unwrap();
                    answer += digit;
                }
            }
        }
    }
    println!("{}", answer);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
