use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut sum = 0;
    let mut sum2 = 0;

    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String

        let mut col: Vec<i32> = vec![];
        let mut rows: Vec<i32> = vec![];

        let mut n = 0;
        for line in lines {
            if let Ok(l) = line {
                
                if l == "" {

                    sum += reflection(&rows, &col);
                    sum2 += reflection_smudge(&rows, &col);

                    n = 0;
                    col = vec![];
                    rows = vec![];
                    continue;
                }

                if col.len() == 0 {
                    col = vec![0;l.len()];
                }

                let mut row: i32 = 0;
                for (i, c) in l.chars().into_iter().enumerate() {
                    if c == '#' {
                        row += 1 << i;
                        col[i] += 1 << n;
                    }
                }

                n += 1;
                rows.push(row);

            }

        }

        sum += reflection(&rows, &col);
        sum2 += reflection_smudge(&rows, &col);
    }

    println!("{}", sum);
    println!("{}", sum2);

}

fn reflection(rows: &Vec<i32>, col: &Vec<i32>) -> i32 {

    let mut sum: i32 = 0;

    // Check Row
    let mut i: isize = 0;
    'outer: while (i as usize) < rows.len() - 1 {

        if rows[i as usize] == rows[i as usize+1] {

            // Check rem
            let mut o = 1;
            while i-o >= 0 && ((i+1+o) as usize) < rows.len() {
                if rows[(i-o) as usize] != rows[(i+1+o) as usize] {
                    i += 1;
                    continue 'outer;
                }
                o += 1;
            }
            sum += 100 * (i as i32 + 1);
        }

        i += 1;
    }

    // Check Columns
    i = 0;
    'outer: while (i as usize) < col.len() - 1 {
        if col[i as usize] == col[i as usize +1] {

           // Check rem
           let mut o = 1;
           while i-o >= 0 && ((i+1+o) as usize) < col.len() {
               if col[(i-o) as usize] != col[(i+1+o) as usize] {
                   i += 1;
                   continue 'outer;
               }
               o += 1;
           }

           sum += i as i32 + 1;
           break;
        }

        i += 1;
    }

    return sum;

}

fn reflection_smudge(rows: &Vec<i32>, col: &Vec<i32>) -> i32 {

    let mut n = 0;
    let mut sum: i32 = 0;

    // Check Row
    let mut i: isize = 0;
    'outer: while (i as usize) < rows.len() - 1 {

        n = (rows[i as usize] ^ rows[i as usize+1]).count_ones();
        if n < 2 {

            // Check rem
            let mut o = 1;
            while i-o >= 0 && ((i+1+o) as usize) < rows.len() {

                if rows[(i-o) as usize] != rows[(i+1+o) as usize] {
                    if n == 1 {
                        i += 1;
                        continue 'outer;
                    }
                    n = (rows[(i-o) as usize] ^ rows[(i+1+o) as usize]).count_ones();
                    if n != 1 {
                        i += 1;
                        continue 'outer;
                    }
                }
                o += 1;
            }
            if n == 1 {
                sum += 100 * (i as i32 + 1);
                break;
            }
        }

        i += 1;
    }

    // Check Columns
    i = 0;
    'outer: while (i as usize) < col.len() - 1 {

        n = (col[i as usize] ^ col[i as usize+1]).count_ones();
        if n < 2 {

           // Check rem
           let mut o = 1;
           while i-o >= 0 && ((i+1+o) as usize) < col.len() {
               if col[(i-o) as usize] != col[(i+1+o) as usize] {
                    if n == 1 {
                        i += 1;
                        continue 'outer;
                    }
                    n = (col[(i-o) as usize] ^ col[(i+1+o) as usize]).count_ones();
                    if n != 1 {
                        i += 1;
                        continue 'outer;
                    }
               }
               o += 1;
           }

           if n == 1 {
            sum += i as i32 + 1;
            break;
           }
        }

        i += 1;

    }

    return sum;

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}