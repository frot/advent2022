use std::fs::File;
use std::io::{self, BufRead};
use queue::Queue;
use std::fmt::Display;

fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename);
    return io::BufReader::new(file.unwrap()).lines();
}

struct Monkey {
    items: Queue<i64>,
    oper: char,
    oper_val: i64,
    test: i64,
    targets: [usize; 2],
    score: i64
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: Queue::<i64>::new(),
            oper: ' ',
            oper_val: 0,
            test: 0,
            targets: [0; 2],
            score: 0
        }
    }

    fn remove(&mut self, div: i64, modulo: i64) -> Option<i64> {
        let i = self.items.dequeue()?;
        self.score += 1;
        let ii = match self.oper {
            '+' => i + self.oper_val,
            '*' => i * self.oper_val,
            '^' => i * i,
            _   => i
        };
        Some((ii/div) % modulo)
    }

    fn throw(&self, i: i64) -> usize {
        if (i % self.test) == 0 {
            self.targets[0]
        }
        else {
            self.targets[1]
        }
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.items.vec().iter().map(|i|format!("{}",i)).collect::<Vec<String>>().join(", ")).unwrap();
        write!(f, ", score: {}", self.score)
    }
}

fn parse(filename: &str) -> Vec<Monkey> {
    let mut lines = read_lines(filename).map(|l| l.unwrap());
    let mut monkeys = Vec::<Monkey>::new();

    while lines.next() != None {
        let mut m = Monkey::new();

        let item_str = lines.next().unwrap();
        let item_split1 = item_str.split(':').last().unwrap();
        for i in item_split1.split(',') {
            m.items.queue(i.trim().parse::<i64>().unwrap()).unwrap();
        }

        let ops_str = lines.next().unwrap();
        let mut ops = ops_str.split(' ').rev();
        if let Ok(val) = ops.next().unwrap().parse::<i64>() {
            m.oper_val = val;
            m.oper = ops.next().unwrap().chars().next().unwrap();
        }
        else {
            m.oper = '^';
        }

        m.test = lines.next().unwrap().split(' ').last().unwrap().parse::<i64>().unwrap();
        m.targets[0] = lines.next().unwrap().split(' ').last().unwrap().parse::<usize>().unwrap();
        m.targets[1] = lines.next().unwrap().split(' ').last().unwrap().parse::<usize>().unwrap();

        monkeys.push(m);
        lines.next();
    }

    return monkeys;
}

fn run_prog(monkeys: &mut Vec<Monkey>) {
    let div = 1;  // First = 3
    let modulo = monkeys.iter().map(|m|m.test).product();

    for _round in 0..10000 {
        for mi in 0..monkeys.len() {
            let m = &mut monkeys[mi];
            let mut q = Queue::<(usize, i64)>::new();
            while let Some(i) = m.remove(div, modulo) {
                q.queue((m.throw(i), i)).unwrap();
            }
            for item in q.vec() {
                let m = &mut monkeys[item.0];
                m.items.queue(item.1).unwrap();
            }
        }
        //println!("Round {}", round);
        //for mi in 0..monkeys.len() {
        //    println!("Monkey {}: {}", mi, &monkeys[mi]);
        //}
    }        
}

fn main() {
    let mut monkeys = parse("input1.txt");
    run_prog(&mut monkeys);

    monkeys.sort_unstable_by_key(|m| m.score);
    monkeys.reverse();
    println!("\n{}", monkeys[0].score*monkeys[1].score);
}
