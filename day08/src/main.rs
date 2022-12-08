use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::max;

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename);
    return io::BufReader::new(file.unwrap()).lines();
}

fn transpose<T: Default + Copy>(m: Vec::<Vec::<T>>) -> Vec::<Vec::<T>>
{
    let mut t = vec![Vec::<T>::with_capacity(m.len()); m[0].len()];
    for r in m {
        for i in 0..r.len() { t[i].push(r[i]); }
    }
    return t;
}

#[allow(unused)]
fn pprint<T: std::fmt::Display>(v: &Vec::<Vec::<T>>) {
    println!("{}", v.iter().map(|r|r.iter().map(|c|format!("[{:>3}]",c)).collect::<Vec<String>>().join(" ")).collect::<Vec<String>>().join("\n"));
}

fn count_height(h: &Vec::<Vec::<u32>>, v: &mut Vec::<Vec::<u32>>) -> u32 {
    let mut count = 0;

    for row in 1..h.len()-1 {
        let mut col = 0;
        let mut top_h = 0;
        let mut top_col = 0;
        while col < h[row].len() {
            if h[row][col] > top_h {
                if v[row][col] == 0 {
                    v[row][col] = 1;
                    count += 1;
                }
                top_h = h[row][col];
                top_col = col;
            }
            col += 1;
        }
        col = h[row].len()-1;
        top_h = 0;
        while col > top_col {
            if h[row][col] > top_h {
                if v[row][col] == 0 {
                    v[row][col] = 1;
                    count += 1;
                }
                top_h = h[row][col];
            }
            col -= 1;
        }
    }

    return count;
}

fn count_score(h: &Vec::<Vec::<u32>>, v: &mut Vec::<Vec::<usize>>) -> usize {
    let mut highscore = 0;

    for row in 1..h.len()-1 {
        let end = h[row].len()-1;
        for col in 1..end {
            let mut cc = col+1;
            while cc<end && h[row][col] > h[row][cc] {
                cc += 1;
            }
            let score = v[row][col] * (cc - col);
            v[row][col] = score;
            highscore = max(score, highscore);

            cc = col-1;
            while cc>0 && h[row][col] > h[row][cc] {
                cc -= 1;
            }
            let score = v[row][col] * (col - cc);
            v[row][col] = score;
            highscore = max(score, highscore);
        }
    }

    //pprint(v);

    return highscore;
}

fn scan(filename: &str) {
    let lines = read_lines(filename).map(|l| l.unwrap());
    let mut height = Vec::<Vec::<u32>>::new();
    for line in lines {
        height.push(line.chars().map(|c| c as u32).collect());
    }

    let mut visible = vec![vec![0; height[0].len()]; height.len()];
    let mut count = 4;
    count += count_height(&height, &mut visible);

    let mut score = vec![vec![1; height[0].len()]; height.len()];
    count_score(&height, &mut score);

    height = transpose(height);
    visible = transpose(visible);
    score = transpose(score);

    count += count_height(&height, &mut visible);
    println!("{}", count);

    let highscore = count_score(&height, &mut score);
    println!("{}", highscore);
}

fn main() {
    scan("input1.txt");
}
