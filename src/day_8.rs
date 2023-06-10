use std::{ collections::HashSet, fs };

pub fn day_8a() -> u64 {
    let contents: Vec<Vec<i8>> = fs
        ::read_to_string("src/data/day8.txt")
        .expect("Should be able to open file")
        .trim()
        .split('\n')
        .map(|row|
            row
                .chars()
                .map(|d| d.to_string().parse::<i8>().unwrap())
                .collect()
        )
        .collect();
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    for i in 1..contents.len() - 1 {
        for (j, dir) in [
            (0, (0, 1)),
            (contents[0].len() - 1, (0, -1)),
        ] {
            let mut pos = (i, j);
            let mut cur_height = -1;
            while cur_height != 9 && pos.1 < contents[0].len() {
                if contents[pos.0][pos.1] > cur_height {
                    cur_height = contents[pos.0][pos.1];
                    visible_trees.insert(pos);
                }
                if pos.1 == 0 && dir.1 == -1 {
                    break;
                }
                pos.1 = ((pos.1 as i32) + dir.1) as usize;
            }
        }
    }
    for j in 1..contents[0].len() - 1 {
        for (i, dir) in [
            (0, (1, 0)),
            (contents.len() - 1, (-1, 0)),
        ] {
            let mut pos = (i, j);
            let mut cur_height = -1;
            while cur_height != 9 && pos.0 < contents.len() {
                if contents[pos.0][pos.1] > cur_height {
                    cur_height = contents[pos.0][pos.1];
                    visible_trees.insert(pos);
                }
                if pos.0 == 0 && dir.0 == -1 {
                    break;
                }
                pos.0 = ((pos.0 as i32) + dir.0) as usize;
            }
        }
    }
    (visible_trees.len() as u64) + 4
}

pub fn day_8b() -> u64 {
    let contents: Vec<Vec<i8>> = fs
        ::read_to_string("src/data/day8.txt")
        .expect("Should be able to open file")
        .trim()
        .split('\n')
        .map(|row|
            row
                .chars()
                .map(|d| d.to_string().parse::<i8>().unwrap())
                .collect()
        )
        .collect();
    let mut max_val = 0;
    for i in 0..contents.len() {
        for j in 0..contents[i].len() {
            let mut product = 1;
            let height = contents[i][j];
            for dir in [
                (0, 1),
                (0, -1),
                (1, 0),
                (-1, 0),
            ] {
                let mut count_line = 0;
                let mut pos = (i, j);
                while
                    pos.0 < contents.len() &&
                    pos.1 < contents[pos.0].len() &&
                    (contents[pos.0][pos.1] < height || pos == (i, j))
                {
                    count_line += 1;
                    if (pos.0 == 0 && dir.0 == -1) || (pos.1 == 0 && dir.1 == -1) {
                        break;
                    }
                    pos.0 = ((pos.0 as i32) + dir.0) as usize;
                    pos.1 = ((pos.1 as i32) + dir.1) as usize;
                }
                if pos.0 >= contents.len() {
                    pos.0 -= 1;
                }
                if pos.1 >= contents[pos.0].len() {
                    pos.1 -= 1;
                }
                if contents[pos.0][pos.1] < height || pos == (i, j) {
                    count_line -= 1;
                }
                product *= count_line;
            }
            max_val = max_val.max(product);
        }
    }
    max_val
}
