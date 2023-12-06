use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Specify the path to the file

    let mut matrix: Vec<Vec<u8>> = vec![];

    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                matrix.push(l.as_bytes().to_vec());
            }
        }
    }

    let mut num: i64 = 0;
    let mut sum: i64 = 0;
    let mut mul: i64 = 0;

    let mut sum2: i64 = 0;

    for (y, line) in matrix.iter().enumerate() {

        for (x, char) in line.iter().enumerate() {

            if *char >= b'0' && *char <= b'9' {
                num *= 10;
                num += (char-b'0') as i64;
                mul |= adjacent_char(&matrix, x, y);
            } else {

                if *char == b'*' {
                    sum2 += adjacent_factors(&matrix, x, y);
                }

                sum += num * mul;
                num = 0;
                mul = 0;
            }
        }

        sum += num * mul;
        num = 0;
        mul = 0;
    }

    println!("{:?}", sum);
    println!("{:?}", sum2);
}

fn adjacent_factors(matrix: &Vec<Vec<u8>>, x: usize, y: usize) -> i64 {

    let directions = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    let mut counter = 0; 
    let mut roots: [(usize, usize);3] = [(0,  0), (0,  0),(0,  0)];

    for &(dx, dy) in &directions {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        // Check if the new coordinates are within bounds
        if valid_coordinates(matrix, new_x, new_y) {

            let c = matrix[new_y as usize][new_x as usize];

            // Check if the character is a special symbol
            if c >= b'0' && c <= b'9' {
                roots[counter] = (find_root(matrix, new_x as usize, new_y as usize), new_y as usize);

                if counter >= 1  {
                    if roots[counter-1] == roots[counter] {
                        counter -= 1;
                    }

                } else if counter == 2 {
                    return 0
                }

                counter += 1;
            }
        }
    }

    if counter == 2 {
        println!("Two tuples");
        return parse_num(matrix, roots[0].0, roots[0].1) * parse_num(matrix, roots[1].0, roots[1].1);
    }

    return 0;

}

fn parse_num(matrix: &Vec<Vec<u8>>, mut x: usize, y: usize) -> i64 {

    let mut num = 0;
    loop {
        let char = matrix[y][x];
        if char >= b'0' && char <= b'9' {
            num *= 10;
            num += (char-b'0') as i64;
            x += 1;
        } else {
            break;
        }

        if x == matrix[0].len() {
            break;
        }

    }

    return num
}

fn find_root(matrix: &Vec<Vec<u8>>, mut x: usize, y: usize) -> usize {

    while x >= 1 {
        x -= 1;
        let c = matrix[y as usize][x];
        if c < b'0' || c > b'9' {
            return x+1;
        }
    }  

    return x
}

fn adjacent_char(matrix: &Vec<Vec<u8>>, x: usize, y: usize) -> i64 {

    let directions = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    // Check all surrounding positions
    for &(dx, dy) in &directions {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        // Check if the new coordinates are within bounds
        if valid_coordinates(matrix, new_x, new_y) {

            let c = matrix[new_y as usize][new_x as usize];

            // Check if the character is a special symbol
            if c != b'.' && (c < b'0' || c > b'9') {
                return 1;
            }
        }
    }

    return 0;
}

fn valid_coordinates(matrix: &Vec<Vec<u8>>, x: isize, y: isize) -> bool {
    return  x >= 0 && x < matrix[0].len() as isize &&
            y >= 0 && y < matrix.len() as isize
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}