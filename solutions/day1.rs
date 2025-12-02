use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
    
    let mut dial = 50;
    let mut part1 = 0;
    let mut part2 = 0;

    let path = Path::new("inputs/day1.dat");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();

    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why);
    }

    for line in s.split("\n") {
        let mut chars = line.chars();
        let letter = chars.next().unwrap();
        let num: i32 = chars.as_str().parse().unwrap_or_else(|_| {
            panic!("bad number in line: {}", line)
        });
        // println!("{} -> letter='{}', number={}", line, letter, num);

        for _i in 0..num {
            dial = (dial + (if letter == 'L' {1} else {-1})) % 100;
            if dial == 0 {
                part2 += 1;
            }
        }
        if dial == 0 {
            part1 += 1;
        }
    }   
    println!("Part 1: {}\nPart 2: {}", part1, part2);

}