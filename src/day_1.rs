use std::fs;
use itertools::Itertools;

pub fn day_1a() -> u64 {
    let contents = fs::read_to_string("src/data/day1.txt").expect("Should be able to open file");
    let elf_amounts = contents.split("\n\n").map(|elf_data| {
        elf_data
            .split('\n')
            .map(|n| {
                match n.parse::<u64>() {
                    Ok(v) => v,
                    _ => 0,
                }
            })
            .sum()
    });
    elf_amounts.max().expect("Array is not empty")
}

pub fn day_1b() -> u64 {
    let contents = fs::read_to_string("src/data/day1.txt").expect("Should be able to open file");
    let elf_amounts = contents.split("\n\n").map(|elf_data| {
        elf_data
            .split('\n')
            .map(|n| {
                match n.parse::<u64>() {
                    Ok(v) => v,
                    _ => 0,
                }
            })
            .sum::<u64>()
    });
    elf_amounts.sorted().rev().take(3).sum()
}