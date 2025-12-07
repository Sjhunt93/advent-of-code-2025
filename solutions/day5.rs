use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


// all numbers are prime...
fn main() {

    
    let path = Path::new("inputs/day5.dat");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();

    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why);
    }

    let (ranges, ingredients) = s.split_once("\n\n").expect("missing blank line separator");
        let ingredients_vals: Vec<i64> = ingredients
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim().parse::<i64>().unwrap())
        .collect();
    
    let mut ranges_pairs: Vec<(i64, i64)> = ranges
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (a, b) = l.split_once('-').expect("invalid range line (expected 'a-b')");
            (a.trim().parse::<i64>().unwrap(), b.trim().parse::<i64>().unwrap())
        })
        .collect();
    
    let mut part1: i64 = 0;
    
    for ing in &ingredients_vals {
        let mut found = 0;
        for (start, end) in &ranges_pairs {
            if ing >= start && ing <= end {
                found = 1;
                break;
            }
            
        }
        part1 += found;
    }
    
    let mut part2: i64 = 0;
    ranges_pairs.sort_by_key(|&(start, _)| start);
    
    // this was my first solution, but it was buggy... forget to track max of the range
    // while read1 < ranges_pairs.len() {
    //     println!("{} {}",  read1, ranges_pairs[read1].0);
    //     let mut read2: usize = read1 + 1;
    //     while read2 < ranges_pairs.len() {
    //         // gap found
    //         if ranges_pairs[read2].0 > ranges_pairs[read1].1 {
    //             read2 -= 1;
    //             println!("breaking as gap found...");
    //             break;
    //         }
    //         println!("extending...");

    //         // if 
    //         read2 += 1;
    //     }
    //     if read2 >= ranges_pairs.len() {
    //         read2 = ranges_pairs.len() - 1;
    //     }
    //     part2 += ranges_pairs[read2].1 - ranges_pairs[read1].0;
    //     println!("range includes {} {} {} --> {}", read1, read2, ranges_pairs[read1].0,  ranges_pairs[read2].1);
    //     read1 = read2 + 1;
    // }
    

    let mut current_start = ranges_pairs[0].0;
    let mut current_end = ranges_pairs[0].1;

    for (start, end) in ranges_pairs.iter().skip(1) {
        if *start <= current_end {
            current_end = std::cmp::max(current_end, *end);
        } else {
            
            part2 += current_end - current_start + 1;
            current_start = *start;
            current_end = *end;
        }
    }    
    part2 += current_end - current_start + 1;

    println!("Part 1: {}\nPart 2: {}", part1, part2);

}