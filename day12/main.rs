use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::{Regex, Replacer};

fn main() {

    let mut sum = 0;
    let mut sum2 = 0;

    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String

        let mut ln = 0;
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

                let pattern = Regex::new(&regex_pattern).unwrap();
                sum += arrangements(&row, &num, "", &pattern);


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

                sum2 += arrangements(&row5, &num5, "", &pattern);
                println!("{}", ln);
                ln += 1;
            }

        }
    }


    println!("{}", sum);
    println!("{}", sum2);

}

fn arrangements(row: &str, num: &[u8], prev: &str, pattern: &Regex) -> i32 {

    let mut n = num[0];

    let s: u8 = num[1..].iter().copied().sum();
    //println!("Sum: {s} Row: {} Len: {}", row.len(), num[1..].len());
    let d: isize = row.len() as isize - (s as isize + num.len() as isize - 1);
    if d <= 0 {
        return 0;
    }

    let end = d as usize;

    //println!("End: {end}");

    let mut sum = 0;

    // Skip .
    let mut i = 0;
    while i < row.len() {
        if row.chars().nth(i).unwrap() != '.' {
            break;
        }
        i += 1;
    } 

    while i < end {

        let mut c = row.chars().nth(i).unwrap();
        let first = c== '#';

        let mut j = i;
        while c == '#' || c == '?' {
            n -= 1;
            j += 1;
            
            if j >= row.len() {
                if n == 0 && num.len() == 1 {
                    let n = num[0] as usize;

                    // let m = prev.to_string() + &row[..i].to_string() + &vec!["#";n].join("") + &row[i+n..].to_string();
                    // if m == "" {
                    //     println!("fail")
                    // }
                    // println!("{}", m.replace("?", "."));
                    sum += 1;
                }
                return sum;
            }

            c = row.chars().nth(j).unwrap();

            if n == 0 {
                if c != '#' {
                    if num.len() == 1 {
                        let res = check_rem(&row[j..]);
                        if res == 1 {
                            // let n = num[0] as usize;
                            // let m =  prev.to_string() + &row[..i].to_string() + &vec!["#";n].join("") + &row[i+n..].to_string();
                            // if m == "" {
                            //     println!("fail")
                            // }
                            sum += 1;
                           //println!("{}", m.replace("?", "."))
                        }
                    } else if j < row.len() - 1 {
                        let n = num[0] as usize;
                        //let m = prev.to_string() + &row[..i].to_string() + &vec!["#";n].join("") + &row[i+n..i+n+1].to_string();
                        let m = "";
                        sum += arrangements(&row[j+1..], &num[1..], &m, pattern);
                    }
                }
                break;
            }
        }

        if first {
            break;
        }

        n = num[0];
        i += 1;
    }

    return sum;
}

fn check_rem(row: &str) -> i32 {
    
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