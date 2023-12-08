use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut ranges: Vec<(i64, i64)> = vec![]; 
    let mut maps: Vec<Vec<[i64; 3]>> = vec![];

    let mut lines = read_lines("./input").unwrap();
    let line = lines.next().unwrap().unwrap(); // First line

    let nums: Vec<&str> = line.split(" ").collect();
    let mut seeds = vec![0; nums.len()-1];

    for i in 0..seeds.len() {
        seeds[i] = nums[i+1].parse::<i64>().unwrap()
    }

    let mut i = 0;
    while i < seeds.len() {
        ranges.push((seeds[i], seeds[i+1]));
        i += 2;
    }

    let mut coordinates = seeds.clone();
    let mut c = coordinates.len();

    let mut map: Vec<[i64; 3]> = vec![];

    for line in lines {
        if let Ok(l) = line {

            if l == "" {
                if !map.is_empty() {
                    maps.push(map);
                }
                map = vec![];
                c = seeds.len();
                continue;
            }

            if !l.chars().nth(0).unwrap().is_ascii_digit() {
                continue;
            }

            let mut m: [i64; 3] = [0;3]; 
            let s: Vec<&str> = l.split(" ").collect();
            for (x, i) in s.iter().enumerate() {
                m[x] = i.parse::<i64>().unwrap();
            }

            map.push(m);

            let mut i = 0;
            while i < c {

                if coordinates[i] >= m[1] && coordinates[i] < m[1] + m[2] {

                    coordinates[i] = m[0] + (coordinates[i] - m[1]);
                    coordinates.swap(i, c-1);
                    seeds.swap(i, c-1);

                    c -= 1;
                    continue;
                }

                i += 1;
            }
        }
    }

    maps.push(map);

    let mut lowest = 0;
    for (i, n) in coordinates.iter().enumerate() {
        if *n < coordinates[lowest] {
            lowest = i;
        }
    }

    println!("{}", seeds[lowest]);
    println!("{}", coordinates[lowest]);

    // Reverse lookup
    for map in &mut maps {
       map.sort_by(|a, b| a[0].cmp(&b[0]));
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut i = 0;

    let tuple = reverse_lookup((0, std::i64::MAX), &ranges, &maps, &mut i);
    println!("{:?}", tuple);
    println!("Iterations: {i}");
}

fn reverse_lookup(range: (i64, i64), ranges: &Vec<(i64, i64)>, maps: &[Vec<[i64; 3]>], num: &mut i32) -> (i64, i64) {

    *num = *num + 1;

    let l = maps.len();

    if l == 0 {

        for c in ranges {

            if c.0 + c.1 < range.0 {
                continue;
            } else if range.0 + range.1 < c.0 {
                break;
            }

            if c.0 > range.0 {
                return (c.0, c.0)
            }

            return (range.0, range.0);
        }

    } else {

        let mut r = (0,0);

        for m in &maps[l-1] {

            if m[0] + m[2] < range.0 {
                continue;
            } else if range.0 + range.1 < m[0] {
                break;
            }

            // Check Beginning
            if m[0] >= range.0 {

                // Check Gap
                if r != (0,0) && r.0 + r.1 < m[0] {
                    r.0 = r.0+r.1;
                    r.1 = m[0] - r.0;
                    let res = reverse_lookup(r, ranges, &maps[0..l-1], num);
                    if res != (0,0) {
                        return res;
                    }
                }

                r.0 = m[0];
            } else {
                r.0 = range.0
            }

            if m[0] + m[2] > range.0 + range.1 {
                r.1 = (range.0 + range.1) - r.0;
            } else {
                r.1 = (m[0] + m[2]) - r.0;
            }
            
            let offset = r.0 - m[0];
            let n_range = (m[1]+offset, r.1);
            let res = reverse_lookup(n_range, ranges, &maps[0..l-1], num);
            if res != (0,0) {
                return (res.0, m[0] + (res.1 - m[1]));
            }

        }
    }
    
    return (0,0);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}