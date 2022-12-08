use std::fs::File;
use std::io::{self, BufRead};

const TOTAL: u32 = 70000000;
const NEEDED: u32 = 30000000;

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename);
    return io::BufReader::new(file.unwrap()).lines();
}

// First
#[allow(unused)]
fn pop1(dirs: &mut Vec::<(u32, String)>) -> u32 {
    let p = dirs.pop().unwrap();
    let i = dirs.len() - 1;
    dirs[i].0 += p.0;
    if p.0 <= 100000 {
        return p.0;
    }
    return 0;
}

#[allow(unused)]
fn scan1(filename: &str) {
    let lines = read_lines(filename).map(|l| l.unwrap());
    let mut dirs = Vec::<(u32, String)>::new();
    let mut sum = 0;

    for line in lines {
        let s: Vec<&str> = line.split(' ').collect();
        if s[0] == "$" {
            if s[1] == "cd" {
                if s[2] == ".." {
                    sum += pop1(&mut dirs);
                }
                else {
                    dirs.push((0, s[2].to_string()));
                }
            }
        }
        else if s[0] != "dir" {
            let i = dirs.len() - 1;
            dirs[i].0 += s[0].parse::<u32>().unwrap();
        }
    }

    while dirs.len() > 1 {
        sum += pop1(&mut dirs);
    }

    println!("sum: {}", sum);
}

fn pop2(dirs: &mut Vec::<(u32, String)>, all_dirs: &mut Vec::<(u32, String)>) {
    let p = dirs.pop().unwrap();
    let i = dirs.len() - 1;
    dirs[i].0 += p.0;
    all_dirs.push(p);
}

fn scan2(filename: &str) {
    let lines = read_lines(filename).map(|l| l.unwrap());
    let mut dirs = Vec::<(u32, String)>::new();
    let mut all_dirs = Vec::<(u32, String)>::new();

    for line in lines {
        let s: Vec<&str> = line.split(' ').collect();
        if s[0] == "$" {
            if s[1] == "cd" {
                if s[2] == ".." {
                    pop2(&mut dirs, &mut all_dirs);
                }
                else {
                    dirs.push((0, s[2].to_string()));
                }
            }
        }
        else if s[0] != "dir" {
            let i = dirs.len() - 1;
            dirs[i].0 += s[0].parse::<u32>().unwrap();
        }
    }

    while dirs.len() > 1 {
        pop2(&mut dirs, &mut all_dirs);
    }

    let top = dirs.pop().unwrap();
    let free = TOTAL - top.0;
    let needed = NEEDED - free;
    println!("total:  {}", top.0);
    println!("free:   {}\nneeded:  {}", free, needed);

    all_dirs.sort();
    let d = all_dirs.iter().find(|d| d.0 >= needed).unwrap();
    println!("{} {}\n", d.0, d.1);

}

fn main() {
    scan2("test1.txt");
    scan2("input1.txt");
}
