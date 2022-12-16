use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::{min, max};
use std::collections::HashMap;

type Dict = HashMap<String, Valve>;

struct Valve {
    name: String,
    rate: i32,
    paths: Vec<String>,
    open: bool,
}

impl Valve {
    fn new(name: &str, rate: i32, paths: Vec<String>) -> Valve {
        Valve { name: name.to_string(), rate: rate, paths: paths, open: false }
    }
}

impl std::fmt::Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Valve {} has flow rate={} and is {}; tunnels lead to valves {}", self.name, self.rate, if self.open {"open"} else {"closed"}, self.paths.join(", "))
    }
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename);
    return io::BufReader::new(file.unwrap()).lines();
}

fn parse(filename: &str) -> Dict {
    let lines = read_lines(filename).map(|l| l.unwrap());
    let mut valves = Dict::new();

    for line in lines {
        let s1 = line.split(&[' ', '=', ';', ','][..]).collect::<Vec<&str>>();
        let s2 = Valve::new(s1[1], s1[5].parse().unwrap(), s1[11..].iter().filter_map(|s|if s.len()>0 {Some(s.to_string())} else {None}).collect());
        valves.insert(s2.to_string(), s2);
    }

    return valves;
}

fn main() {
    let s = parse("test1.txt");
}
