use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut lines = read_lines("./input").unwrap();
    let line = lines.next().unwrap().unwrap(); // First line

    // Get Instruction Set
    let mut index: usize = 0;
    let mut instructions: Vec<usize> = vec![0; line.len()];

    for (i, c) in line.chars().enumerate() {
        if c == 'R' {
            instructions[i] = 1;
        }
    }

    let mut map: HashMap<String, [String;2]> = HashMap::new();

    for line in lines {
        if let Ok(l) = line {

            if l == "" {
                continue;
            }

            let node = l[0..3].to_string();
            let left = l[7..10].to_string();
            let right = l[12..15].to_string();

            map.insert(node, [left, right]);
        }
    }


    let mut ghosts: Vec<String> = vec![];

    for key in map.keys() {
        if key.chars().nth(2).unwrap() == 'A' {
            ghosts.push(key.clone())
        }
    }

    let mut jumps = 0;
    let mut pos: String = "AAA".to_string();
    while pos != "ZZZ" {
        jumps += 1;

        let lr = map.get(&pos).unwrap();
        pos = lr[instructions[index]].clone();

        index += 1;
        index %= instructions.len();
    }

    println!("{}", jumps);


    let mut lcm: u64 = 1;

    let mut i = 0;
    while i < ghosts.len() {

        jumps = 0;
        index = 0;

        pos = ghosts[i].clone();

        let mut e: u8 = 0;
        let mut prev: Vec<String> = vec![];

        print!("{pos}");

        loop {
            jumps += 1;
            
            let lr = map.get(&pos).unwrap();
            pos = lr[instructions[index]].clone();
            
            if pos.chars().nth(2).unwrap() == 'Z' {
                print!(" -- {jumps} --> {pos}");
                if prev.contains(&pos) {
                    if e == 1 {
                        lcm = jumps * lcm / gcd(jumps, lcm);
                        println!();
                        break;
                    }
                    e += 1;
                }
                jumps = 0;
                prev.push(pos.clone());
            }

            index += 1;
            index %= instructions.len();
        }
        i += 1;
    }

    println!("{lcm}")

}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}