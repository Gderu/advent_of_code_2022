use std::{ fs, collections::HashSet };

use itertools::Itertools;

pub fn day_6a() -> usize {
    let contents = fs::read_to_string("src/data/day6.txt").expect("Should be able to open file");
    contents
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .find_position(|w| HashSet::<&char>::from_iter(w.iter()).len() == 4)
        .expect("Should be a first position that satisfies this").0 + 4
}

pub fn day_6b() -> usize {
    let contents = fs::read_to_string("src/data/day6.txt").expect("Should be able to open file");
    contents
        .chars()
        .collect::<Vec<_>>()
        .windows(14)
        .find_position(|w| HashSet::<&char>::from_iter(w.iter()).len() == 14)
        .expect("Should be a first position that satisfies this").0 + 14
}