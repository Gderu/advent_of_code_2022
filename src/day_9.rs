use std::{ fs, collections::{ HashMap, HashSet } };

pub fn day_9a() -> u64 {
    let contents = fs::read_to_string("src/data/day9.txt").expect("Should be able to open file");
    let instructions: Vec<(char, u32)> = contents
        .trim()
        .split('\n')
        .map(|line| {
            let mut split_line = line.split(' ');
            (
                split_line.next().unwrap().chars().next().unwrap(),
                split_line.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    let translate_dir = HashMap::from([
        ('R', (1, 0)),
        ('L', (-1, 0)),
        ('U', (0, 1)),
        ('D', (0, -1)),
    ]);
    let mut tail_pos: (i32, i32) = (0, 0);
    let mut head_pos = (0, 0);
    let mut positions_been = HashSet::from([(0, 0)]);
    for (dir_letter, n) in instructions {
        let dir = translate_dir[&dir_letter];
        for _ in 0..n {
            head_pos.0 += dir.0;
            head_pos.1 += dir.1;
            let difference = (head_pos.0 - tail_pos.0, head_pos.1 - tail_pos.1);

            if difference.0.abs() > 1 || difference.1.abs() > 1 {
                if difference.0 != 0 {
                    tail_pos.0 += difference.0 / difference.0.abs();
                }
                if difference.1 != 0 {
                    tail_pos.1 += difference.1 / difference.1.abs();
                }
            }
            positions_been.insert(tail_pos);
        }
    }
    positions_been.len() as u64
}

pub fn day_9b() -> u64 {
    let contents = fs::read_to_string("src/data/day9.txt").expect("Should be able to open file");
    let instructions: Vec<(char, u32)> = contents
        .trim()
        .split('\n')
        .map(|line| {
            let mut split_line = line.split(' ');
            (
                split_line.next().unwrap().chars().next().unwrap(),
                split_line.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    let translate_dir = HashMap::from([
        ('R', (1, 0)),
        ('L', (-1, 0)),
        ('U', (0, 1)),
        ('D', (0, -1)),
    ]);
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut positions_been = HashSet::from([(0, 0)]);
    for (dir_letter, n) in instructions {
        let dir = translate_dir[&dir_letter];
        for _ in 0..n {
            rope[0].0 += dir.0;
            rope[0].1 += dir.1;
            for i in 0..rope.len() - 1 {
                let head = rope[i];
                let mut tail = &mut rope[i + 1];
                let difference = (head.0 - tail.0, head.1 - tail.1);

                if difference.0.abs() > 1 || difference.1.abs() > 1 {
                    if difference.0 != 0 {
                        tail.0 += difference.0 / difference.0.abs();
                    }
                    if difference.1 != 0 {
                        tail.1 += difference.1 / difference.1.abs();
                    }
                }
            }

            positions_been.insert(rope[rope.len() - 1]);
        }
    }
    positions_been.len() as u64
}
