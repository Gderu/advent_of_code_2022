use std::fs;
use itertools::Itertools;
use regex::Regex;

pub fn day_5a() -> String {
    let re = Regex::new(r"\d+").unwrap();
    let contents = fs::read_to_string("src/data/day5.txt").expect("Should be able to open file");
    let starting_pos = contents.split("\n\n").nth(0).expect("Should be the starting pos");
    let num_piles = (starting_pos.find('\n').expect("Piles should not be empty") + 1) / 4;
    let mut piles = vec![vec![]; num_piles];
    for line in starting_pos.split('\n') {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c.is_digit(10) {
                break;
            }
            if c != ' ' {
                piles[i].push(c);
            }
        }
    }

    let moves = contents
        .split("\n\n")
        .nth(1)
        .expect("Should be the moves")
        .split('\n')
        .map(|line| {
            re.find_iter(line)
                .filter_map(|mat| mat.as_str().parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .filter_map(|line| {
            match line.len() {
                0 => None,
                _ => Some(line),
            }
        })
        .collect::<Vec<_>>();
    
    for line in moves {
        let moved_elements = piles[line[1] - 1].drain(0..line[0]).rev().collect::<Vec<_>>();
        piles[line[2] - 1].splice(..0, moved_elements);
    }
    piles.iter().map(|pile| pile[0]).join("")
}


pub fn day_5b() -> String {
    let re = Regex::new(r"\d+").unwrap();
    let contents = fs::read_to_string("src/data/day5.txt").expect("Should be able to open file");
    let starting_pos = contents.split("\n\n").nth(0).expect("Should be the starting pos");
    let num_piles = (starting_pos.find('\n').expect("Piles should not be empty") + 1) / 4;
    let mut piles = vec![vec![]; num_piles];
    for line in starting_pos.split('\n') {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c.is_digit(10) {
                break;
            }
            if c != ' ' {
                piles[i].push(c);
            }
        }
    }

    let moves = contents
        .split("\n\n")
        .nth(1)
        .expect("Should be the moves")
        .split('\n')
        .map(|line| {
            re.find_iter(line)
                .filter_map(|mat| mat.as_str().parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .filter_map(|line| {
            match line.len() {
                0 => None,
                _ => Some(line),
            }
        })
        .collect::<Vec<_>>();
    
    for line in moves {
        let moved_elements = piles[line[1] - 1].drain(0..line[0]).collect::<Vec<_>>();
        piles[line[2] - 1].splice(..0, moved_elements);
    }
    piles.iter().map(|pile| pile[0]).join("")
}