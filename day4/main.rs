use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut sum = 0;

    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            let mut points = 1;
            if let Ok(l) = line {
                
                let numbers: Vec<&str> = l[7..].split('|').collect();

                let mut i = 0;
                while i < numbers[1].len() {
                    if numbers[0].contains(&numbers[1][i..i+3]) {
                        points = points << 1;
                    }
                    i += 3
                }

            }

            sum += points >> 1;
        }
    }


    print!("{}", sum);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}