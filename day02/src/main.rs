use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
#[macro_use] extern crate maplit;

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename);
    return io::BufReader::new(file.unwrap()).lines();
}

fn score(hmap: HashMap<&str, i32>) -> i32 {
    let lines = read_lines("input1.txt");
    let mut score = 0;

    for line in lines {
        if let Ok(l) = line {
            score += hmap.get(&*l).unwrap();
        }
    }

    println!("{}", score);

    return score;
}

fn main() {
    // X rock, Y paper, Z scissors
    let first = hashmap!{
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
    };

    // X lose, Y draw, Z win
    let second = hashmap!{
        "A X" => 0 + 3,
        "A Y" => 3 + 1,
        "A Z" => 6 + 2,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 0 + 2,
        "C Y" => 3 + 3,
        "C Z" => 6 + 1,
    };

    score(first);
    score(second);
}
