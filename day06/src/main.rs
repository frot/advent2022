use std::fs::File;
use std::io::{self, BufRead};

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename);
    return io::BufReader::new(file.unwrap()).lines();
}

fn scan(filename: &str, ln: usize) {
    let lines = read_lines(filename).map(|l| l.unwrap());

    for line in lines {
        'pos: for pos in ln..line.len() {
            let mut s: Vec<char> = line[pos-ln..pos].chars().collect();
            s.sort();
            for i in 1..s.len() {
                if s[i] == s[i-1] {
                    continue 'pos;
                }
            }
            println!("{} {}", &line[pos-4..pos], pos);
            break;
        }
    }
}

fn main() {
    scan("input1.txt", 4);
    scan("input1.txt", 14);
}
