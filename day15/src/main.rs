use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::{min, max};
use std::collections::HashSet;

type Pt = (i32, i32);

struct Sensor {
    pos: Pt,
    beacon: Pt,
    dist: i32
}

impl Sensor {
    fn new(pos: Pt, beacon: Pt) -> Sensor {
        Sensor { pos: pos, beacon: beacon, dist: (pos.0-beacon.0).abs()+(pos.1-beacon.1).abs() }
    }

    fn import(pos_x: &str, pos_y: &str, bc_x: &str, bc_y: &str) -> Sensor {
        Sensor::new((pos_x.parse().unwrap(), pos_y.parse().unwrap()),
                    (bc_x.parse().unwrap(), bc_y.parse().unwrap()))
    }
}

impl std::fmt::Display for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Sensor at x={}, y={}: closest beacon is at x={}, y={}", self.pos.0, self.pos.1, self.beacon.0, self.beacon.1)
    }
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename);
    return io::BufReader::new(file.unwrap()).lines();
}

fn parse(filename: &str) -> Vec<Sensor> {
    let lines = read_lines(filename).map(|l| l.unwrap());
    let mut sensors = Vec::<Sensor>::new();

    for line in lines {
        let s1 = line.split(&[' ', '=', ',', ':'][..]).collect::<Vec<&str>>();
        let s2 = Sensor::import(s1[3], s1[6], s1[13], s1[16]);
        sensors.push(s2);
    }

    return sensors;
}

fn add_line(v: &mut Vec<Pt>, mut ln: Pt) -> Vec<Pt> {
    let mut v_out = Vec::<Pt>::new();
    for line in v {
        if ln.0 <= line.1 && ln.1 >= line.0 {
            ln.0 = min(ln.0, line.0);
            ln.1 = max(ln.1, line.1);
        }
        else {
            v_out.push(*line);
        }
    }
    v_out.push(ln);
    return v_out;
}

fn eval_line(ss: &Vec<Sensor>, row: i32) -> Vec<Pt> {
    let mut rlist = Vec::<Pt>::new();
    for r in ss {
        let d = r.dist - (row-r.pos.1).abs();
        if d >= 0 {
            rlist = add_line(&mut rlist, (r.pos.0-d, r.pos.0+d));
        }
    }
    return rlist;
}

fn count_covered(ss: &Vec<Sensor>, row: i32) -> i32 {
    let rlist = eval_line(ss, row);
    let mut rset = HashSet::<i32>::new();
    let mut cnt = 0;
    for ln in rlist {
        cnt += 1+ln.1-ln.0;
    }
    for r in ss {
        if r.beacon.1 == row {
            rset.insert(r.beacon.1);
        }
    }
    return cnt - rset.len() as i32;
}

fn find_free_row(ss: &Vec<Sensor>, limit: i32, row: i32) -> Option<i32> {
    let rlist = eval_line(ss, row);
    for ln in rlist {
        if ln.0 > 0 && ln.0 < limit {
            return Some(ln.0-1);
        }
        if ln.1 > 0 && ln.1 < limit {
            return Some(ln.1+1);
        }
    }
    return None;
}

fn find_free(ss: &Vec<Sensor>, limit: i32) -> Option<Pt> {
    for row in 0..=limit {
        if let Some(col) = find_free_row(ss, limit, row) {
            return Some((col, row));
        }
    }
    return None;
}

fn main() {
    let s = parse("input1.txt");
    let n = count_covered(&s, 2000000);
    println!("{}", n);

    let (x, y) = find_free(&s, 4000000).unwrap();
    println!("{}, {}", x, y);
    let f = 4000000*x as u64 + y as u64;
    println!("{}", f);
}
