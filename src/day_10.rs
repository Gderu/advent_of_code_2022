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
    // let contents = fs::read_to_string("src/data/day10.txt").expect("Should be able to open file");
    let contents = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    let instructions: Vec<&str> = contents.trim().split('\n').collect();
    let mut cycle: i32 = 0;
    let mut register = 1;
    for line in instructions {
        if line == "noop" {
            if (cycle - register).abs() % 40 <= 1 {
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
            if (cycle - register).abs() % 40 <= 1 {
                screen[cycle as usize] = "#";
            } else {
                screen[cycle as usize] = ".";
            }
            cycle += 1;
            if (cycle - register).abs() % 40 <= 1 {
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
