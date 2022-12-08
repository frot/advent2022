use std::fs::File;
use std::io::{self, BufRead};

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename);
    return io::BufReader::new(file.unwrap()).lines();
}

#[allow(dead_code)]
fn pprint(s: &Vec::<Vec::<char>>) {
    for v in s {
        for c in v {
            print!(" {}", c)
        }
        println!("");
    }
    println!("");
}

fn tprint(s: &Vec::<Vec::<char>>) {
    for v in s {
        print!("{}", v.last().unwrap());
    }
    println!("");
}

fn first(filename: &str) {
    let mut lines = read_lines(filename).map(|l| l.unwrap());
    let mut stacks = Vec::<Vec::<char>>::new();

    while let Some(line) = lines.next() {
        let mut it = line.chars();
        if it.next() == None {
            break;
        }
        for (i, c) in it.step_by(4).enumerate() {
            if c.is_numeric() {
                break;
            }
            if i >= stacks.len() {
                stacks.push(Vec::new());
            }
            if c.is_alphabetic() {
                stacks[i].insert(0, c);
            }
        }
    }

    while let Some(line) = lines.next() {
        let nums: Vec<&str> = line.split(' ').collect();
        let num = nums[1].parse::<usize>().unwrap();
        let from = nums[3].parse::<usize>().unwrap();
        let to = nums[5].parse::<usize>().unwrap();
        /* First
        for _n in 0..num {
            let c = stacks[from-1].pop().unwrap();
            stacks[to-1].push(c);
        }
        */
        /* Second */
        let ln = stacks[from-1].len();
        let s = &stacks[from-1][ln-num..ln].to_vec();
        stacks[to-1].extend_from_slice(s);
        stacks[from-1].truncate(ln-num);
    }

    tprint(&stacks);
}

fn main() {
    first("input1.txt");
}
