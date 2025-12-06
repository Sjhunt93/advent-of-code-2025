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
            // exit early
            if row[index_of_largest] == 9 {
                break;
            }
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
    
    let mut part1: i64 = 0;
    let mut part2: i64 = 0;
    let max: usize = 12;
    for y in 0..height {
        let mut sum: i64 = 0;
        let mut start: usize = 0;
        for i in 0..max {
            let pow = (max - i - 1) as u32;
            let multiplier = 10_i64.pow(pow);

            let end = width - (max - i) + 1;
            let index_of_largest = index_of_largest_value_in_range(&grid[y], start, end);
            // println!("{} {} {}:{} ", i, end, index_of_largest, grid[y][index_of_largest]);
            start = index_of_largest + 1;
            sum += (grid[y][index_of_largest] as i64) * multiplier;
        }
        println!(" {} ", sum);
        part2 += sum;
        
        
    


    }

    println!("Part 1: {}\nPart 2: {}", part1, part2);

}