use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn parse_grid(s: &str) -> Vec<Vec<char>> {
    let rows: Vec<Vec<char>> = s.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| line.trim().chars().collect())
        .collect();

    let height = rows.len();
    let width = rows[0].len();

    // create padded grid with '.' border (1 cell on each side => +2 rows and +2 cols)
    let mut padded = vec![vec!['.'; width + 2]; height + 2];
    for y in 0..height {
        for x in 0..width {
            padded[y + 1][x + 1] = rows[y][x];
        }
    }
    return padded
}

fn is_in_range(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    return x >= 0 && x < width && y < height && y >= 0;
}

fn adjaceny_count(grid: &Vec<Vec<char>>, x: usize, y: usize, target: char) -> usize {
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < width && ny >= 0 && ny < height {
                if grid[ny as usize][nx as usize] == target {
                    count += 1;
                }
            }
        }
    }
    return count;
}

// all numbers are prime...
fn main() {
    // return;
    
    let path = Path::new("inputs/day4.dat");
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

    // println!("Is in range: {} {}", is_in_range(&grid, 3, 4), is_in_range(&grid, -1, 0));
    let mut part1: i64 = 0;
    for y in 1..height-1 {
        for x in 1..width-1 {
            if grid[y][x] == '@' {
                if adjaceny_count(&grid, x, y, '.') > 4 {
                    part1 += 1;
                }
            }
        }
        
    }
    println!("Part 1: {}", part1);







    // println!("Part 1: {}\nPart 2: {}", part1, part2);

}