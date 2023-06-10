use std::fs;

pub fn day_4a() -> u64 {
    let contents = fs::read_to_string("src/data/day4.txt").expect("Should be able to open file");
    contents.split('\n').map(|line| {
        if line.len() == 0 {
            return 0;
        }
        let first_range: Vec<u64> = line.split(',').nth(0).expect("Should have two parts").split('-').map(|x| x.to_string().parse().expect("Should always be a number")).collect();
        let second_range: Vec<u64> = line.split(',').nth(1).expect("Should have two parts").split('-').map(|x| x.to_string().parse().expect("Should always be a number")).collect();
        if first_range[0] <= second_range[0] && second_range[1] <= first_range[1] || second_range[0] <= first_range[0] && first_range[1] <= second_range[1] {
            1
        } else {
            0
        }
    }).sum()
}

pub fn day_4b() -> u64 {
    let contents = fs::read_to_string("src/data/day4.txt").expect("Should be able to open file");
    contents.split('\n').map(|line| {
        if line.len() == 0 {
            return 0;
        }
        let first_range: Vec<u64> = line.split(',').nth(0).expect("Should have two parts").split('-').map(|x| x.to_string().parse().expect("Should always be a number")).collect();
        let second_range: Vec<u64> = line.split(',').nth(1).expect("Should have two parts").split('-').map(|x| x.to_string().parse().expect("Should always be a number")).collect();
        if second_range[0] <= first_range[0] && first_range[0] <= second_range[1] || first_range[0] <= second_range[0] && second_range[0] <= first_range[1] {
            1
        } else {
            0
        }
    }).sum()
}