use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// fn

fn can_be_split_2(i: &str) -> bool {
    // if i.len() <= 1 {
    //     return false;
    // }
    let s = i.to_string();
    let mid = s.len() / 2;
    let (left, right) = s.split_at(mid);
    return left == right;
    // if left != right {
    //     return can_be_split_2(left) && can_be_split_2(right);
    // }
    // return true;
}
fn is_correct(s: &str) -> bool {

    if s.len() == 9 {
        return s[0..3] == s[3..6] && s[0..3] == s[6..9];
    }
    else if s.len() == 6 {
        let condition1 = s[0..2] == s[2..4] && s[4..6] == s[0..2];
        let condition2 = can_be_split_2(s);
        return condition1 || condition2;
    }
    else if s.len() == 10 {
        let condition1 = s[0..2] == s[2..4] && s[4..6] == s[6..8] && s[0..2] == s[8..10] && s[8..10] == s[6..8];
        let condition2 = can_be_split_2(s);
        return condition1 || condition2;
    }
    else if s.len() % 2 == 0 {
        return can_be_split_2(s);
    }
    else if s.len() >= 2 {
        let bytes = s.as_bytes();
        for i in 1..bytes.len() {
            if bytes[i - 1] != bytes[i] {
                return false;
            }
        }
        return true;
    }
    println!("**** len={} {}", s.len(), s);
    return false;
}

fn is_correct_v2(s: &str) -> bool {

    if s.len() == 9 {
        return s[0..3] == s[3..6] && s[0..3] == s[6..9];
    }
    else if s.len() == 6 {
        let condition1 = s[0..2] == s[2..4] && s[4..6] == s[0..2];
        let condition2 = can_be_split_2(s);
        return condition1 || condition2;
    }
    else if s.len() == 10 {
        let condition1 = s[0..2] == s[2..4] && s[4..6] == s[6..8] && s[0..2] == s[8..10] && s[8..10] == s[6..8];
        let condition2 = can_be_split_2(s);
        return condition1 || condition2;
    }
    // 4,8, 12
    else if s.len() % 2 == 0 {
        return can_be_split_2(s);
    }
    // 3,5,7,11
    else if s.len() >= 2 {
        let bytes = s.as_bytes();
        for i in 1..bytes.len() {
            if bytes[i - 1] != bytes[i] {
                return false;
            }
        }
        return true;
    }

    return false;
}

// all numbers are prime...
fn main() {
    println!("{}", can_be_split_2("123123123"));
    println!("{}", is_correct("145145145"));
    println!("{}", is_correct("1111111"));
    println!("{}", is_correct("2121212121"));
    // return;
    
    let path = Path::new("inputs/day2.dat");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();

    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why);
    }
    let mut part1: i64 = 0;
    let mut part2: i64 = 0;
    for line in s.split(",") {
        
        let (a, b) = line.split_once('-').unwrap();
        let start: i64 = a.parse().unwrap();
        let end:   i64 = b.parse().unwrap();
        // println!("start={} end={}", start, end);
        for i in start..=end {
            let s = i.to_string();
            let mid = s.len() / 2;
            let (left, right) = s.split_at(mid);
            // println!("{} -> left='{}' right='{}'", i, left, right);
            if left == right {
                part1 += i;
            }
            is_correct(&s);
        }
        for i in start..=end {
            let s = i.to_string();
            if is_correct(&s) {
                // println!("correct: {} {}", s, s.len());
                part2 += i;
            }
        }
        
        
    }   
    println!("Part 1: {}\nPart 2: {}", part1, part2);

}