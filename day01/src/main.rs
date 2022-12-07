use std::fs::File;
use std::io::{self, BufRead};

fn first(filename: &str) -> Vec<i32> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut elves: Vec<i32> = Vec::new();
    let mut sum = 0;

    for line in lines {
        if let Ok(l) = line {
            if l.len() > 0 {
                sum += l.parse::<i32>().unwrap();
            } else {
                elves.push(sum);
                sum = 0;
            }
        }
    }
    elves.push(sum);

    println!("{}", elves.iter().max().unwrap());

    return elves;
}

fn second(filename: &str) {
    let mut elves = first(filename);
    elves.sort();
    elves.reverse();

    let total = elves.iter().take(3).sum::<i32>();

    println!("{}", total);
}

fn main() {
    //first("input1.txt");
    second("input1.txt");
}
