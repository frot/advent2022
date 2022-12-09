use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use num::clamp;

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename);
    return io::BufReader::new(file.unwrap()).lines();
}

#[allow(unused)]
fn pprint<T: std::fmt::Display>(v: &Vec::<Vec::<T>>) {
    println!("{}", v.iter().rev().map(|r|r.iter().map(|c|format!("{}",c)).collect::<Vec<String>>().join(" ")).collect::<Vec<String>>().join("\n"));
    println!("----");
}

#[allow(unused)]
fn move_rope1(hpos: &mut (i32, i32), tpos: &mut (i32, i32), dir: &str) -> (i32, i32) {
    match dir {
        "L" => { hpos.0-=1; if (tpos.0-hpos.0)>1 { tpos.0-=1; tpos.1=hpos.1; }},
        "R" => { hpos.0+=1; if (hpos.0-tpos.0)>1 { tpos.0+=1; tpos.1=hpos.1; }},
        "D" => { hpos.1-=1; if (tpos.1-hpos.1)>1 { tpos.1-=1; tpos.0=hpos.0; }},
        "U" => { hpos.1+=1; if (hpos.1-tpos.1)>1 { tpos.1+=1; tpos.0=hpos.0; }},
        _   => ()
    };
    return (tpos.0, tpos.1);
}

fn move_rope2(pos: &mut Vec<(i32, i32)>, dir: &str) -> (i32, i32) {
    match dir {
        "L" => pos[0].0-=1,
        "R" => pos[0].0+=1,
        "D" => pos[0].1-=1,
        "U" => pos[0].1+=1,
        _   => ()
    };
    for i in 1..pos.len() {
        if (pos[i].0-pos[i-1].0) > 1 {
            pos[i].0 -= 1;
            pos[i].1 += clamp(pos[i-1].1 - pos[i].1, -1, 1);
        }
        else if (pos[i-1].0-pos[i].0)>1 {
            pos[i].0 += 1;
            pos[i].1 += clamp(pos[i-1].1 - pos[i].1, -1, 1);
        }
        else if (pos[i].1-pos[i-1].1)>1 {
            pos[i].1 -= 1;
            pos[i].0 += clamp(pos[i-1].0 - pos[i].0, -1, 1);
        }
        else if (pos[i-1].1-pos[i].1)>1 {
            pos[i].1 += 1;
            pos[i].0 += clamp(pos[i-1].0 - pos[i].0, -1, 1);
        }
    }
 
    let tpos = pos.last().unwrap();
    return (tpos.0, tpos.1);
}

fn rope(filename: &str) {
    let lines = read_lines(filename).map(|l| l.unwrap());
    let mut path = HashSet::<(i32, i32)>::new();
    let mut pos = vec![(0, 0); 10];

    path.insert(pos[0]);
    for line in lines {
        let mut s = line.split(' ');
        let dir = s.next().unwrap();
        let num = s.next().unwrap().parse::<i32>().unwrap();
        for _ in 0..num {
            path.insert(move_rope2(&mut pos, dir));
        }
    }

    println!("{}", path.len());
}

fn main() {
    rope("input1.txt");
}
