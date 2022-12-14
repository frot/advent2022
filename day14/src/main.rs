use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::{min, max};

type Pt = (usize, usize);

const WIDTH: usize = 700;
const HEIGHT: usize = 600;
const BG: char = '0';
const LN: char = '4';
const SN: char = '7';

struct Map {
    grid: Vec<Vec<char>>,
    min: Pt,
    max: Pt
}

impl Map {
    fn new() -> Map {
        Map { grid: vec![vec![BG; WIDTH]; HEIGHT], min: (WIDTH,0), max: (0,0) }
    }

    fn draw(&mut self, p0: &Pt, p1: &Pt) {
        self.min.0 = min(self.min.0, min(p0.0, p1.0));
        self.min.1 = min(self.min.1, min(p0.1, p1.1));
        self.max.0 = max(self.max.0, max(p0.0, p1.0));
        self.max.1 = max(self.max.1, max(p0.1, p1.1));

        if p0.0 == p1.0 {
            for row in p0.1..=p1.1 {
                self.grid[row][p0.0] = LN;
            }
            for row in p1.1..=p0.1 {
                self.grid[row][p0.0] = LN;
            }
        }
        else if p0.1 == p1.1 {
            for col in p0.0..=p1.0 {
                self.grid[p0.1][col] = LN;
            }
            for col in p1.0..=p0.0 {
                self.grid[p0.1][col] = LN;
            }
        }
        else {
            panic!("Not supported");
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "P2\n{} {}\n7\n{}", 1+self.max.0-self.min.0, 1+self.max.1-self.min.1, self.grid[self.min.1..=self.max.1].iter().map(|r|r[self.min.0..=self.max.0].iter().map(|c|format!("{} ",c)).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("\n"))
    }
}

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename);
    return io::BufReader::new(file.unwrap()).lines();
}

fn to_pt(s: &str) -> Pt {
    let mut ss = s.split(',');
    (ss.next().unwrap().parse::<usize>().unwrap(),
     ss.next().unwrap().parse::<usize>().unwrap())
}

fn parse(filename: &str) -> Map {
    let lines = read_lines(filename).map(|l| l.unwrap());
    let mut map = Map::new();

    for line in lines {
        let pt_vec = line.split(" -> ").map(|s|to_pt(s)).collect::<Vec<Pt>>();
        let mut p0 = &pt_vec[0];
        for p1 in &pt_vec[1..] {
            map.draw(p0, p1);
            p0 = p1;
        }
    }

    // Second
    map.draw(&(0, map.max.1+2), &(WIDTH-1, map.max.1+2));

    return map;
}

fn simulate(filename: &str) -> i32 {
    let mut map = parse(filename);
    let mut step = 0;

    loop {
        let mut s: Pt = (500,0);
        while s.1 <= map.max.1 {
            if map.grid[s.1+1][s.0] == BG {
                s.1 += 1;
            }
            else if map.grid[s.1+1][s.0-1] == BG {
                s.0 -= 1;
                s.1 += 1;
            }
            else if map.grid[s.1+1][s.0+1] == BG {
                s.0 += 1;
                s.1 += 1;
            }
            else {
                break;
            }
        }
        if s.1 <= map.max.1 {
            map.grid[s.1][s.0] = SN;
            if s.1 == 0 {
                println!("{}\n", map);
                return step+1;
            }
        }
        else {
            println!("{}\n", map);
            return step;
        }
        step += 1;
    }
}

fn main() {
    let i = simulate("input1.txt");
    eprintln!("{}", i);
}
