use std::fs::File;
use std::io::{self, BufRead};
use queue::Queue;

type Pt = (usize, usize);

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename);
    return io::BufReader::new(file.unwrap()).lines();
}

#[allow(unused)]
fn pprint<T: std::fmt::Display>(v: &Vec::<Vec::<T>>) {
    println!("{}", v.iter().map(|r|r.iter().map(|c|format!("{}",c)).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("\n"));
    println!("----");
}

fn find(m: &mut Vec<Vec<char>>, c: char, repl: char) -> Option<Pt> {
    for (row, rc) in m.iter_mut().enumerate() {
        if let Some(col) = rc.iter().position(|&x|x==c) {
            rc[col] = repl;
            return Some((row, col));
        }
    }
    return None;
}

fn check_add(q: &mut Queue<Pt>, map: &Vec<Vec<char>>, dist: &mut Vec<Vec<i32>>, c0: char, d0: i32, p: Pt) {
    if p.0 < map.len() && p.1 < map[0].len() && map[p.0][p.1] >= c0 && dist[p.0][p.1] > d0 {
        dist[p.0][p.1] = d0;
        q.queue(p).unwrap();
    }
}

#[allow(unused_variables)]
fn climb(filename: &str) {
    let lines = read_lines(filename).map(|l| l.unwrap());
    let mut map = lines.map(|l|l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut dist = vec![vec![i32::MAX; map[0].len()]; map.len()];
    let mut q = Queue::<Pt>::new();
    let p_start = find(&mut map, 'S', 'a').unwrap();
    let p_end = find(&mut map, 'E', 'z').unwrap();

    dist[p_end.0][p_end.1] = 0;
    q.queue(p_end).unwrap();

    while let Some(p) = q.dequeue() {
        //if p == p_start {  // First
        if map[p.0][p.1] == 'a' {  // Second
            println!("{}", dist[p.0][p.1]);
            break;
        }

        let c0 = ((map[p.0][p.1] as u8) - 1) as char;
        let d0 = dist[p.0][p.1] + 1;

        if p.0 > 0 {
            check_add(&mut q, &map, &mut dist, c0, d0, (p.0-1, p.1));
        }
        if p.1 > 0 {
            check_add(&mut q, &map, &mut dist, c0, d0, (p.0, p.1-1));
        }
        check_add(&mut q, &map, &mut dist, c0, d0, (p.0+1, p.1));
        check_add(&mut q, &map, &mut dist, c0, d0, (p.0, p.1+1));
    }
}

fn main() {
    climb("input1.txt");
}
