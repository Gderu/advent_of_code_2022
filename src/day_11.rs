use std::{ fs, mem };
use num::Integer;
use itertools::Itertools;

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: u64,
    throw_true: u64,
    throw_false: u64,
    num_inspected: u64,
    can_be_relieved: bool,
    lcm: Option<u64>,
}

impl Monkey {
    fn new(as_string: &str, can_be_relieved: bool) -> Monkey {
        let split_to_lines: Vec<String> = as_string
            .split('\n')
            .map(|line| line.trim().to_owned())
            .collect();
        let items = split_to_lines[1]
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        let mut operation_str = split_to_lines[2]
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(' ')
            .map(|x| x.to_string());
        let sign = operation_str.nth(3).unwrap();
        let other_num = operation_str.next().unwrap();
        let operation = Box::new(move |old| {
            let other = if other_num == "old" { old } else { other_num.parse().unwrap() };
            match sign.as_str() {
                "+" => old + other,
                "*" => old * other,
                _ => panic!("Should only be addition or multiplication"),
            }
        });
        let test = split_to_lines[3]
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let throw_true = split_to_lines[4]
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let throw_false = split_to_lines[5]
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        Monkey {
            items,
            operation,
            test,
            throw_true,
            throw_false,
            num_inspected: 0,
            can_be_relieved,
            lcm: None,
        }
    }

    fn throw(&mut self) -> Vec<(u64, u64)> {
        if self.items.len() == 0 {
            return vec![];
        }
        let old_items = mem::replace(&mut self.items, vec![]);
        self.num_inspected += old_items.len() as u64;

        let res = old_items
            .into_iter()
            .map(|mut item| {
                item = (*self.operation)(item);
                if self.can_be_relieved {
                    item = item / 3;
                }
                if let Some(lcm) = self.lcm.as_ref() {
                    item %= lcm;
                }
                if item % self.test == 0 {
                    (self.throw_true, item)
                } else {
                    (self.throw_false, item)
                }
            })
            .collect();

        res
    }

    fn add(&mut self, item: u64) {
        self.items.push(item);
    }

    fn add_lcm(&mut self, lcm: u64) {
        self.lcm = Some(lcm);
        for item in &mut self.items {
            *item = *item % lcm;
        }
    }
}

pub fn day_11a() -> u64 {
    let contents = fs::read_to_string("src/data/day11.txt").expect("Should be able to open file");
    let mut monkey_list: Vec<Monkey> = contents
        .trim()
        .split("\n\n")
        .map(|monkey| Monkey::new(monkey, true))
        .collect();
    for _ in 0..20 {
        for i in 0..monkey_list.len() {
            let items_thrown = monkey_list[i].throw();
            for (target, item) in items_thrown {
                monkey_list[target as usize].add(item);
            }
        }
    }
    let sorted_num_inspected: Vec<u64> = monkey_list
        .iter()
        .map(|monkey| monkey.num_inspected)
        .sorted()
        .collect();
    sorted_num_inspected[sorted_num_inspected.len() - 1] *
        sorted_num_inspected[sorted_num_inspected.len() - 2]
}

pub fn day_11b() -> u64 {
    let contents = fs::read_to_string("src/data/day11.txt").expect("Should be able to open file");
    let mut monkey_list: Vec<Monkey> = contents
        .trim()
        .split("\n\n")
        .map(|monkey| Monkey::new(monkey, false))
        .collect();

    let mut lcm = 1;
    for monkey in &monkey_list {
        lcm = lcm.lcm(&monkey.test);
    }
    for monkey in &mut monkey_list {
        monkey.add_lcm(lcm);
    }
    for _ in 0..10000 {
        for i in 0..monkey_list.len() {
            let items_thrown = monkey_list[i].throw();
            for (target, item) in items_thrown {
                monkey_list[target as usize].add(item);
            }
        }
    }
    let sorted_num_inspected: Vec<u64> = monkey_list
        .iter()
        .map(|monkey| monkey.num_inspected)
        .sorted()
        .collect();
    sorted_num_inspected[sorted_num_inspected.len() - 1] *
        sorted_num_inspected[sorted_num_inspected.len() - 2]
}
