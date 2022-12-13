use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use lazy_static::lazy_static;

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename);
    return io::BufReader::new(file.unwrap()).lines();
}

#[allow(unused)]
fn pprint<T: std::fmt::Display>(v: &Vec::<Vec::<T>>) {
    println!("{}", v.iter().map(|r|r.iter().rev().map(|c|format!("{} ",c)).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("\n"));
    println!("----");
}

fn parse(s: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+|[\[\]]").unwrap();
    }
    let mut v = RE.find_iter(s).map(|m|m.as_str()).collect::<Vec<&str>>();
    v.reverse();

    return v;
}

fn cmp(a: &Vec<&str>, b: &Vec<&str>) -> Ordering {
    let mut expr1 = a.clone();
    let mut expr2 = b.clone();
    while expr1.len() > 0 {
        if expr2.len() == 0 {
            return Ordering::Greater;
        }
        let m1 = expr1.pop().unwrap();
        let m2 = expr2.pop().unwrap();
        if m1 != m2 {
            if m1 == "[" && m2 != "]" {
                expr2.push("]");
                expr2.push(m2);
                continue;
            }
            if m1 == "]" {
                return Ordering::Less;
            }
            if m2 == "[" && m1 != "]" {
                expr1.push("]");
                expr1.push(m1);
                continue;
            }
            if m2 == "]" {
                return Ordering::Greater;
            }

            let n1 = m1.parse::<i32>().unwrap();
            let n2 = m2.parse::<i32>().unwrap();
            return n1.cmp(&n2);
        }
    }

    return Ordering::Less;
}

#[allow(unused)]
fn validate(filename: &str) -> i32 {
    let mut lines = read_lines(filename).map(|l| l.unwrap());
    let mut step = 1;
    let mut sum = 0;

    loop {
        let l1 = lines.next().unwrap();
        let x1 = parse(&l1);
        let l2 = lines.next().unwrap();
        let x2 = parse(&l2);

        if cmp(&x1, &x2) == Ordering::Less {
            sum += step;
        }
        step += 1;
        if lines.next() == None { break; }
    }

    return sum;
}

#[allow(unused)]
fn sort(filename: &str) -> usize {
    let lines = read_lines(filename).map(|l|l.unwrap()).collect::<Vec<String>>();
    let mut list = lines.iter().filter_map(|l| if l.len() > 0 { Some(parse(&l)) } else { None }).collect::<Vec<Vec<&str>>>();

    list.push(parse("[[2]]"));
    list.push(parse("[[6]]"));

    list.sort_unstable_by(cmp);

    let div1 = parse("[[2]]");
    let div2 = parse("[[6]]");
    let pos1 = 1 + list.iter().position(|p|p==&div1).unwrap();
    let pos2 = 1 + list.iter().position(|p|p==&div2).unwrap();

    return pos1*pos2;
}

fn main() {
    let i = validate("input1.txt");
    println!("{}", i);

    let i = sort("input1.txt");
    println!("{}", i);
}
