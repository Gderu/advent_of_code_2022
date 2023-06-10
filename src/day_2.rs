use std::fs;


pub fn day_2a() -> u64 {
    let contents = fs::read_to_string("src/data/day2.txt").expect("Should be able to open file");
    contents.split('\n').map(|line| {
        let split_line: Vec<&str> = line.split(' ').collect();
        if split_line.len() != 2 {
            return 0;
        }
        let (other, you) = (split_line[0].chars().next().unwrap() as u64 - 'A' as u64, split_line[1].chars().next().unwrap() as u64 - 'X' as u64);
        let result = (3 + other - you) % 3;
        let points = match result {
            0 => 3,
            1 => 0,
            2 => 6,
            _ => panic!("Isn't possible")
        };
        points + you + 1
    }).sum()
}

pub fn day_2b() -> u64 {
    let contents = fs::read_to_string("src/data/day2.txt").expect("Should be able to open file");
    contents.split('\n').map(|line| {
        let split_line: Vec<&str> = line.split(' ').collect();
        if split_line.len() != 2 {
            return 0;
        }
        let (other, you) = (split_line[0].chars().next().unwrap() as u64 - 'A' as u64, split_line[1].chars().next().unwrap() as u64 - 'X' as u64);
        match you {
            0 => (3 + other - 1) % 3 + 1,
            1 => other + 4,
            2 => (3 + other + 1) % 3 + 7,
            _ => panic!("Isn't possible")
        }
    }).sum()
}