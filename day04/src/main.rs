use std::fs::File;
use std::io::{self, BufRead};

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename);
    return io::BufReader::new(file.unwrap()).lines();
}

fn first(filename: &str) -> u32 {
    let lines = read_lines(filename);
    let mut score = 0;

    for line in lines {
        if let Ok(l) = line {
            let nums: Vec<u32> = l.split(&['-', ','][..]).map(|n| n.parse::<u32>().unwrap()).collect();
            let mut i = 0;
            while i < nums.len() {
                if (nums[i] >= nums[i+2] && nums[i+1] <= nums[i+3]) ||
                   (nums[i+2] >= nums[i] && nums[i+3] <= nums[i+1]) {
                    score += 1;
                }
                i += 4;
            }
        }
    }

    println!("{}", score);

    return score;
}

fn second(filename: &str) -> u32 {
    let lines = read_lines(filename);
    let mut score = 0;

    for line in lines {
        if let Ok(l) = line {
            let nums: Vec<u32> = l.split(&['-', ','][..]).map(|n| n.parse::<u32>().unwrap()).collect();
            let mut i = 0;
            while i < nums.len() {
                if nums[i+1] >= nums[i+2] && nums[i] <= nums[i+3] {
                    score += 1;
                }
                i += 4;
            }
        }
    }

    println!("{}", score);

    return score;
}

fn main() {
    first("input1.txt");
    second("input1.txt");
}
