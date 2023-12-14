use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use num_bigint::{BigUint, ToBigUint};
use num_traits::cast::ToPrimitive;

fn main() {

    let mut sum = 0;
    let mut sum2 = 0;

    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String

        for line in lines {
            if let Ok(l) = line {
                
                let col: Vec<&str> = l.split(' ').collect();

                let row = col[0];
                let mut num: Vec<u8> = vec![];
                col[1].split(',').for_each(|x| num.push(x.parse::<u8>().unwrap()));
                
                let mut regex_pattern: String = "[^#]*".to_owned();
                for n in &num {
                    regex_pattern = format!("{}#{{{}}}[^#]*", regex_pattern, n);
                }

                sum += arrangements(&row, &num);

                // Part 2
                let row5 = format!("{}?{}?{}?{}?{}", row, row, row, row, row);
                let mut num5: Vec<u8> = vec![0;num.len()*5];

                let mut i = 0;
                while i < num.len() {
                    num5[i] = num[i];
                    num5[num.len()+i] = num[i];
                    num5[2*num.len()+i] = num[i];
                    num5[3*num.len()+i] = num[i];
                    num5[4*num.len()+i] = num[i];

                    i += 1;
                }

                sum2 += arrangements(&row5, &num5);
            }

        }
    }

    println!("{}", sum);
    println!("{}", sum2);

}

fn arrangements(row: &str, num: &[u8]) -> i128 {

    let s: u8 = num[1..].iter().copied().sum();
    let end: isize = row.len() as isize - (s as isize + num.len() as isize - 1);
    if end <= 0 {
        return 0;
    }

    let mut sum: i128 = 0;

    // Skip .
    let mut dots = 0;
    while dots < row.len() {
        if row.chars().nth(dots).unwrap() != '.' {
            break;
        }
        dots += 1;
    } 

    // Count '?'
    let mut h: i16 = 0; // #
    let mut q = 0; // ?
    let mut i: usize = dots;
    while (i as usize) < row.len() {
        match row.chars().nth(i).unwrap() {
            '#' => {h = 1; break},
            '.' => break,
            _ => q += 1
        }
        i += 1;
    }

    if h == 0 && q != 0 {
        let mut n = 0;
        while n < num.len() {
            let c = questionmark_combinations((i-dots) as i128, &num[0..n+1]);
            if c == 0 {
                break;
            }
            if n == num.len() - 1 {
                sum += c * check_rem(&row[i..]);
                break;
            }

            if i < row.len() {
                sum += c * arrangements(&row[i+1..], &num[n+1..]);
            }
            
            n += 1
        }

        // Try Skipping this block
        if i < row.len() {
            sum += arrangements(&row[i+1..], num);
        }
        return sum
    }

    // Count '#'
    if h == 1 {

        i += 1;
        while i < row.len() {
            match row.chars().nth(i).unwrap() {
                '#' => h += 1,
                _ => break
            }
            i += 1;
        }

        // Combinations
        let mut n = 0;
        while n < num.len() {

            if (num[n] as i16) < h {
                n += 1;
                continue;
            }
            
            let mut c: i128 = 1;
            let shift = num[n] as i16 - h;

            // Shift # Match
            let mut s = 0;
            while s <= shift {


                // Valid?
                let offset = i + s as usize;
                if offset > row.len() || row.chars().nth(offset-1).unwrap() == '.' {
                    break;
                }

                if offset < row.len() {
                    if row.chars().nth(offset).unwrap() == '#' {
                        s += 1;
                        continue;
                    }
                } else if offset > row.len() {
                    break;
                }

                let q_rem = q - (shift-s);
                if q_rem < 0 {
                    s += 1;
                    continue;
                }

                // Check Prev ? Block Combinations
                if n > 0 {
                    c = questionmark_combinations((q_rem - 1) as i128, &num[..n]);
                    if c == 0 {
                        s += 1;
                        continue;
                    }
                }

                if n == num.len() - 1 {
                    sum += c * check_rem(&row[offset..]);
                    s += 1;
                    continue;
                }
    
                if offset < row.len() {
                    sum += c * arrangements(&row[offset+1..], &num[n+1..]);
                }


                s += 1;
            }

            n += 1;
        }

    }
    
    return sum;
}

fn questionmark_combinations(amount: i128, num: &[u8]) -> i128 {

    let mut c: i128;
    let gaps: i128 = num.len() as i128 - 1;
    let sum: u8 = num.iter().copied().sum();
    let mut size = sum as i128 + gaps;
    
    if amount < size {
        return 0;
    }

    c = (amount - size) + 1;
    if num.len() == 1 {
        return c;
    }

    size += 1;
    let mut n = 1; 
    while size <= amount {
        c += ((amount - size) + 1) * combinations(n, gaps);
        size += 1;
        n += 1;
    }

    return c;
}

fn factorial(n: i128) -> BigUint {
    if n == 0 || n == 1 {
        return 1.to_biguint().unwrap();
    }
    n.to_biguint().unwrap() * factorial(n - 1)
}

fn combinations(n: i128, x: i128) -> i128 {
    return (factorial(n+x-1) / (factorial(n) * factorial(x-1))).to_i128().unwrap();
}

fn check_rem(row: &str) -> i128 {
    
    let mut i = 0;
    while i < row.len() {
        if row.chars().nth(i).unwrap() == '#' {
            return 0;
        }
        i += 1;
    }

    return 1
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}