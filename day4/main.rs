use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut sum = 0;
    let mut sum2 = 0;

    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String

        let mut ring: [i64; 10] = [1;10];
        let mut r: usize = 0;
        
        for line in lines {

            let mut points = 1;
            let mut amount = 0;
            if let Ok(l) = line {
                
                let numbers: Vec<&str> = l[7..].split('|').collect();

                let mut i = 0;
                while i < numbers[1].len() {
                    if numbers[0].contains(&numbers[1][i..i+3]) {
                        amount += 1;
                        points = points << 1;
                    }
                    i += 3
                }

            }

            let copies = ring[r];
            sum2 += copies;
            ring[r] = 1;

            let mut i = r;
            while amount > 0 {
                i += 1;
                i %= 10;
                ring[i] += copies;
                amount -= 1;
            }

            r += 1;
            r %= 10;

            sum += points >> 1;
        }
    }


    println!("{}", sum);
    println!("{}", sum2);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}