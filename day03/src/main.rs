use std::fs::File;
use std::io::{self, BufRead};

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename);
    return io::BufReader::new(file.unwrap()).lines();
}

fn char_to_prio(c: char) -> u32 {
    let i = c as u32;
    if 'a' <= c && c <= 'z' {
        return 1 + i - 'a' as u32;
    }
    if 'A' <= c && c <= 'Z' {
        return 27 + i - 'A' as u32;
    }
    return 0;
}

fn first(filename: &str) -> u32 {
    let lines = read_lines(filename);
    let mut score = 0;

    for line in lines {
        if let Ok(l) = line {
            let (l1, l2) = l.split_at(l.len()/2);
            let mut c1: Vec<char> = l1.chars().collect();
            c1.sort();
            let mut c2: Vec<char> = l2.chars().collect();
            c2.sort();
            let mut i = 0;
            for c in c1 {
                while i<c2.len() && c2[i]<c { i+=1; }
                if c2[i] == c {
                    let p = char_to_prio(c);
                    score += p;
                    break;
                }
            }
        }
    }

    println!("{}", score);

    return score;
}

fn second(filename: &str) -> u32 {
    let lines: Vec<String> = read_lines(filename).map(|l| l.unwrap()).collect();
    let mut score = 0;
    let mut j = 0;

    while j < lines.len() {
        let mut c1: Vec<char> = lines[j].chars().collect();
        c1.sort();

        let mut c2: Vec<char> = lines[j+1].chars().collect();
        let mut i2 = 0;
        c2.sort();

        let mut c3: Vec<char> = lines[j+2].chars().collect();
        let mut i3 = 0;
        c3.sort();

        for c in c1 {
            while i2<c2.len() && c2[i2]<c { i2+=1; }
            if c2[i2] == c {
                while i3<c3.len() && c3[i3]<c { i3+=1; }
                if c3[i3] == c {
                    let p = char_to_prio(c);
                    score += p;
                    break;
                }
            }
        }
        j += 3;
    }

    println!("{}", score);

    return score;
}

fn main() {
    first("input1.txt");
    second("input1.txt");
}
