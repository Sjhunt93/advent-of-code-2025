use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


// all numbers are prime...
fn is_blank_column(col: usize, rows: &Vec<Vec<char>>) -> bool {
    if col >= rows[0].len() {
        return true;
    }
    for row in rows {
        if row[col] != ' ' {
            return false;
        }
    }
    return true;
}
fn get_numbers_for_part1(col_start: usize, col_end: usize, rows: &Vec<Vec<char>>) -> Vec::<i64> {
    let mut results = Vec::<i64>::new();
    for y in 0..rows.len()-1 {
        let mut num_str = String::new();
        for x in col_start..col_end {
            num_str.push(rows[y][x]);
        }
        results.push(num_str.trim().parse::<i64>().unwrap());
    }
    return results;
}

fn get_numbers_for_part2(col_start: usize, col_end: usize, rows: &Vec<Vec<char>>) -> Vec::<i64> {
    let mut results = Vec::<i64>::new();
    for x in col_start..col_end {
        let mut num_str = String::new();
        for y in 0..rows.len()-1 {
            num_str.push(rows[y][x]);
        }
        results.push(num_str.trim().parse::<i64>().unwrap());
    }
    return results;
}

fn main() {

    
    let path = Path::new("inputs/day6.dat");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why);
    }

    let mut rows: Vec<Vec<char>> = s
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    let width = rows[0].len();
    let height = rows.len();
    

    let mut part1: i64 = 0;
    let mut part2: i64 = 0;
    let mut range_start: usize = 0;
    
    for col in (0..width+1) {
        if is_blank_column(col, &rows) {
            let op: char = rows[height-1][range_start];
            println!("Range from {} to {}: op: {}", range_start, col, op);
            let format_1 = get_numbers_for_part1(range_start, col, &rows);
            let format_2 = get_numbers_for_part2(range_start, col, &rows);
            
            if op == '+' {
                part1 += format_1.iter().sum::<i64>();
                part2 += format_2.iter().sum::<i64>();
            } else if op == '*' {
                part1 += format_1.iter().product::<i64>();
                part2 += format_2.iter().product::<i64>();
            }
            
            range_start = col + 1;
        }

        

    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}