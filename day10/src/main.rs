use std::fs::File;
use std::io::{self, BufRead};

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename);
    return io::BufReader::new(file.unwrap()).lines();
}

#[allow(unused)]
fn pprint<T: std::fmt::Display>(v: &Vec::<T>) {
    println!("{}", v.chunks(40).map(|r|r.iter().map(|c|format!("{}",c)).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("\n"));
    println!("----");
}

fn run_prog(filename: &str) {
    let lines = read_lines(filename).map(|l| l.unwrap());
    let mut cycle = 1;
    let mut register = 1;
    let mut sum = 0;
    let mut crt = vec!['.'; 240];
    for line in lines {
        for ll in line.split(' ') {
            if ((cycle-20) % 40) == 0 {
                println!("{}: {} = {}", cycle, register, cycle*register);
                sum += cycle*register;
            }

            let pos = cycle-1;
            let xpos = pos % 40;
            if xpos >= (register-1) && xpos <= (register+1) {
                crt[pos as usize] = '#';
            }

            if let Ok(n) = ll.parse::<i32>() {
                register += n;
            }
            cycle += 1;
        }
    }

    println!("sum: {}", sum);
    pprint(&crt);
}

fn main() {
    run_prog("input1.txt");
}
