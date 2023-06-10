use std::fs;

use itertools::Itertools;

pub fn day_10a() -> i32 {
    let contents = fs::read_to_string("src/data/day10.txt").expect("Should be able to open file");
    let cycles_to_count = [20, 60, 100, 140, 180, 220];
    let mut total_sum = 0;
    let instructions: Vec<&str> = contents.trim().split('\n').collect();
    let mut cycle = 0;
    let mut register = 1;
    for line in instructions {
        if line == "noop" {
            cycle += 1;
            if cycles_to_count.contains(&cycle) {
                total_sum += cycle * register;
            }
        } else {
            let split_line: Vec<&str> = line.split(' ').collect();
            if split_line[0] != "addx" {
                panic!("Should always be addx");
            }
            let add_to_register: i32 = split_line[1].parse().unwrap();
            if cycles_to_count.contains(&(cycle + 1)) {
                total_sum += (cycle + 1) * register;
            } else if cycles_to_count.contains(&(cycle + 2)) {
                total_sum += (cycle + 2) * register;
            }
            cycle += 2;
            register += add_to_register;
        }
    }
    println!("{}", cycle);
    total_sum
}

pub fn day_10b() -> String {
    let mut screen = [""; 240];
    let contents = fs::read_to_string("src/data/day10.txt").expect("Should be able to open file");
    let instructions: Vec<&str> = contents.trim().split('\n').collect();
    let mut cycle: i32 = 0;
    let mut register = 1;
    for line in instructions {
        if line == "noop" {
            if (cycle % 40 - register).abs() <= 1 {
                screen[cycle as usize] = "#";
            } else {
                screen[cycle as usize] = ".";
            }
            cycle += 1;
        } else {
            let split_line: Vec<&str> = line.split(' ').collect();
            if split_line[0] != "addx" {
                panic!("Should always be addx");
            }
            let add_to_register: i32 = split_line[1].parse().unwrap();
            if (cycle % 40 - register).abs() <= 1 {
                screen[cycle as usize] = "#";
            } else {
                screen[cycle as usize] = ".";
            }
            cycle += 1;
            if (cycle % 40 - register).abs() <= 1 {
                screen[cycle as usize] = "#";
            } else {
                screen[cycle as usize] = ".";
            }
            cycle += 1;
            register += add_to_register;
        }
    }
    format!("\n{}", screen.into_iter().chunks(40).into_iter().map(|mut chunk| chunk.join("")).join("\n"))
}
