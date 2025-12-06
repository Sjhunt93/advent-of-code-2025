use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn parse_grid(s: &str) -> Vec<Vec<u8>> {
    s.lines()
     .filter(|l| !l.trim().is_empty())
     .map(|line| {
         line.trim()
             .chars()
             .map(|c| c.to_digit(10).expect("non-digit in input") as u8)
             .collect()
     })
     .collect()
}

fn index_of_largest_value_in_range(row: &Vec<u8>, start: usize, end: usize) -> usize {

    let mut index_of_largest = start;
    for x in start..end {
        if row[x] > row[index_of_largest] {
            index_of_largest = x;
        }
    }
    return index_of_largest

}

// all numbers are prime...
fn main() {
    // return;
    
    let path = Path::new("inputs/day3.dat");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();

    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why);
    }

    let grid = parse_grid(&s);
    if grid.is_empty() {
        println!("empty input");
        return;
    }

    let height = grid.len();
    let width = grid[0].len();
    println!("grid size: {} x {}", width, height);

    // example: access column x, row y
    // let x: usize = 3;
    // let y: usize = 2;
    // if y < height && x < width {
    //     println!("grid[{}][{}] = {}", x, y, grid[y][x]);
    // }
    
    let mut part1: i64 = 0;
    let mut part2: i64 = 0;

    for y in 0..height {
        let mut index_of_largest = index_of_largest_value_in_range(&grid[y], 0, width-1);
        let mut index_of_largest2 = index_of_largest_value_in_range(&grid[y], index_of_largest+1, width);
        part1 += (grid[y][index_of_largest] as i64) * 10 + (grid[y][index_of_largest2] as i64);
        // for x in 0..width {
        //     print!("{}", grid[y][x]);
        //     if grid[y][x] > grid[y][index_of_largest] && x < width-1 {
        //         index_of_largest = x;
        //     }
        // }
        print!(" --> {}{}", grid[y][index_of_largest], grid[y][index_of_largest2]);
        println!();
    }




    println!("Part 1: {}\nPart 2: {}", part1, part2);

}